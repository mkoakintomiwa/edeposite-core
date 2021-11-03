use definitions::*;
use variables::*;
use functions::*;
use mysql::*;
use mysql::prelude::*;
use indexmap::*;


pub fn config(db_name: &str, db_user: &str, db_password: &str, db_host: &str, db_port: &str)->String{
    format!("mysql://{}:{}@{}:{}/{}",db_user,db_password,db_host,db_port,db_name)
}


pub fn localhost_config(db_name: &str)->String{
    config(db_name, DB_USER, DB_PASSWORD, DB_HOST, DB_PORT)
}


pub async fn portal_config(db_alias: &str, portal_id: &str)->String{
    let school = portal_properties(portal_id).await;
    let db_user = school.settings.db_user;
    let db_password = school.settings.db_password;
    let db_host = school.ssh.host;
    let mut db_name = school.settings.db_name;

    if db_alias==DB_FILES{
        db_name = format!("{}_files",db_name);
    }else if db_alias==DB_SCANNER{
        db_name = format!("{}_scanner",db_name);
    }else if db_alias==DB_SEGMENTS{
        db_name = format!("{}_segments",db_name);
    }

    config(db_name.as_str(), db_user.as_str() , db_password.as_str(), db_host.as_str(), DB_PORT)
}



pub fn conn(db_name: &str, db_user: &str, db_password: &str, db_host: &str, db_port: &str)->Pool{
    let url = format!("mysql://{}:{}@{}:{}/{}",db_user,db_password,db_host,db_port,db_name);
    let opts = Opts::from_url(&url).unwrap();
    Pool::new(opts).unwrap()
}


pub fn conn_pool(db_name: &str, db_user: &str, db_password: &str, db_host: &str, db_port: &str)->Pool{
    let opts = Opts::from_url(&config(db_name, db_user, db_password, db_host, db_port)).unwrap();
    Pool::new(opts).expect("Error in connection")
}


pub fn pool(config: String)->Pool{
    let opts = Opts::from_url(&config).unwrap();
    Pool::new(opts).expect("Error in connection")
}



pub fn get_pooled_conn(pool: &Pool)->PooledConn{
    pool.get_conn().expect(CONNECTION_ERROR)
}


pub fn execute(query: &str,parameters: Vec<String>, conn_pool: &Pool){
    get_pooled_conn(conn_pool).exec_drop(query, parameters).unwrap();
}

pub fn fetch(query: &str,parameters: Vec<String>, conn_pool: &Pool)->Vec<mysql::Row>{
    get_pooled_conn(conn_pool).exec(query, parameters).unwrap()
}


pub fn fetch_one(query: &str,parameters: Vec<String>,db_conn: &Pool)->mysql::Row{
    fetch(query, parameters, db_conn).remove(0)
}


pub fn _conn(db_name: &str)->Pool{
    conn(db_name,DB_USER,DB_PASSWORD,DB_HOST,DB_PORT)
}




pub struct RowAction<'a>{
    db_parameters: DBParameters<'a>
}


impl <'a>RowAction<'a>{


    pub fn statement(&self)->String{

        let columns = &self.db_parameters.columns;

        let mut s=String::from(" WHERE ");
        let mut i=0;



        for (column,_column_value) in columns{

            if i==0{
                s = format!(r#" {} `{}`=? "#,s,column.trim());
            }else{
                s = format!(r#" {} AND `{}`=? "#,s,column.trim());
            }
            i+=1;
        }

        let query_append = "";
        return format!("{} {}",s,query_append);
    }


    pub fn fetch(&self)->Vec<mysql::Row>{
        let s: String = format!(r#"SELECT * FROM `{}` {} "#,self.db_parameters.table_name,self.statement());
        
        fetch(s.as_str(), indexmap_values(&self.db_parameters.columns), &self.db_parameters.conn_pool)
    }



    pub fn fetch_one(&self)->mysql::Row{
        self.fetch().remove(0)
    }



    pub fn exists(&self)->bool{

        if self.db_parameters.count.is_none(){
            return self.fetch().len()>0;
        }else{
            return false;
        }

    }



    pub fn delete(&self)->&Self{
        let _fields: Vec<String> = self.db_parameters.conn_pool.get_conn().expect(CONNECTION_ERROR).exec(format!("DELETE FROM `{}` {}",self.db_parameters.table_name,self.statement()), indexmap_values(&self.db_parameters.columns)).unwrap();
        &self
    }



    pub fn update(&self)->&Self{

        let s = self.statement();

        let d = indexmap_keys(&self.db_parameters.update).len();

        let mut bb="";
        let mut bbk: String;

        let k = indexmap_keys(&self.db_parameters.update);
        let _v = indexmap_values(&self.db_parameters.update);

        let vk = indexmap_values(&self.db_parameters.update);
        let vv = indexmap_values(&self.db_parameters.columns);

        for g in 0..d{

            if g==d-1{
                bbk = format!("{} `{}`=? ",bb,k[g]);
                bb = bbk.as_str();
            }else{
                bbk = format!("{} `{}`=?, ",bb,k[g]);
                bb = bbk.as_str();
            }
        }

        let _fields: Vec<String> =self.db_parameters.conn_pool.get_conn().expect(CONNECTION_ERROR).exec(format!("UPDATE `{}` SET {} {}",self.db_parameters.table_name,bb,s),vec_merge(&vk,&vv)).unwrap();
        &self
    }



    pub fn insert(&self)->&Self{


        let mut p="(";
        let mut c="(";
        let mut f=0;

        let mut cs: String;
        let mut ps: String;


        for (column_name,_column_value) in &self.db_parameters.columns{
            if f==self.db_parameters.columns.len()-1{
                cs=format!("{} ?",c);
                ps=format!("{} `{}`",p,column_name.trim());

                c = cs.as_str();
                p = ps.as_str();

            }else{
                cs = format!("{} ?,",c);
                ps =format!("{} `{}`,",p,column_name.trim());

                c = cs.as_str();
                p = ps.as_str();
            }

            f+=1;
        }


        ps=format!("{} )",p);
        cs=format!("{} )",c);

        c = cs.as_str();
        p = ps.as_str();

        self.db_parameters.conn_pool.get_conn().expect(CONNECTION_ERROR).exec::<String,String,Vec<String>>(format!("INSERT INTO `{}` {} VALUES {}",self.db_parameters.table_name,p,c),indexmap_values(&self.db_parameters.columns)).unwrap();
        &self
    }



    pub fn insert_once(&self)->&Self{

        if !self.exists(){
            self.insert();
        }
        &self
    }


}



pub fn rowaction(db_parameters_with_columns: DBParametersWithColumns)->RowAction{
    let db_parameters = DBParameters{
        conn_pool: db_parameters_with_columns.conn_pool,
        table_name: db_parameters_with_columns.table_name,
        columns: db_parameters_with_columns.columns,
        update: indexmap!{},
        count: None,
        query_append: None
    };
    RowAction{
        db_parameters
    }
}



pub fn rowaction_update(db_parameters_with_update: DBParametersWithUpdate)->RowAction{
    let db_parameters = DBParameters{
        conn_pool: db_parameters_with_update.conn_pool,
        table_name: db_parameters_with_update.table_name,
        columns: db_parameters_with_update.columns,
        update: indexmap!{},
        count: None,
        query_append: None
    };
    RowAction{
        db_parameters
    }
}





pub fn rowaction_all(db_parameters: DBParameters)->RowAction{
    RowAction{
        db_parameters
    }
}




pub fn unique_from_db(table_name: &str, column_name: &str, length: usize,conn_pool: &Pool, context: &str)->String{
    let  mut content: String;
    loop{
        if context=="digits"{
            content = random_digits(length)
        }else{
            content = random_characters(length);
        }

        let rows = fetch(&format!("SELECT * FROM `{}` WHERE `{}` = ?", table_name, column_name), _tv(vec![&content]), conn_pool);

        if rows.len() == 0{
            return content;
        }
    }
    
}



pub fn unique_digits_from_db(table_name: &str, column_name: &str, length: usize,conn_pool: &Pool)->String{
    unique_from_db(table_name, column_name, length, conn_pool, "digits")
}



pub fn unique_characters_from_db(table_name: &str, column_name: &str, length: usize,conn_pool: &Pool)->String{
    unique_from_db(table_name, column_name, length, conn_pool, "characters")
}