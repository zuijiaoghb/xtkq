use axum::{
    routing::get,
    Json, Router,
    extract::Path,
    debug_handler,
    Error,
    response::Html,
};

use oracle::{
    Connection,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Clone,Debug)]
struct OrgMember{
    id : i64,
    name : String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct OaSigndate{
    member_code : String,
    member_name : String,
    sign_date : String,
}

#[debug_handler]
//async fn getOrgMember(Path(member_id): Path<u64>) -> Json<Vec<OrgMember>> {
async fn getOaSigndate(Path(signdate): Path<String>) -> Json<Vec<OaSigndate>> {    
    let connec = Connection::connect("v3xuser","Jtdj_0795","1.5/orcl");    
    let conn = match connec {
        Ok(co) => co, 
        Err(error) => panic!("conn problem :{}", error),
    };
    
    //let sql = "select id, name from org_member where name = :name";
    //let sql = "select to_char(id) id, name from org_member";
   /* let sql = "select b.member_code,b.member_name, to_char(a.sign_date/(1000*60*60*24)+to_date('1970-01-01 08:00:00','yyyy/mm/dd hh24:mi:ss'),'yyyy/mm/dd hh24:mi:ss') as sign_date from att_mobile_record a
	   left join att_member b on b.member_id=a.member_id
	   where a.account_id = -1771325801755686689 and to_char(a.sign_date/(1000*60*60*24)+to_date('1970-01-01 08:00:00','yyyy/mm/dd hh24:mi:ss'),'yyyy-mm-dd')= :signdate
	   order by a.sign_date desc";
       */

    let sql = "select b.member_code,b.member_name, to_char(a.sign_date/(1000*60*60*24)+to_date('1970-01-01 08:00:00','yyyy/mm/dd hh24:mi:ss'),'yyyy/mm/dd hh24:mi:ss') as sign_date from att_mobile_record a
       left join att_member b on b.member_id=a.member_id
       where a.account_id = -1771325801755686689 and to_char(a.sign_date/(1000*60*60*24)+to_date('1970-01-01 08:00:00','yyyy/mm/dd hh24:mi:ss'),'yyyy-mm-dd')= :signdate
union
select  b.member_code,b.member_name,to_char(a.record_datetime/(1000*60*60*24)+to_date('1970-01-01 08:00:00','yyyy/mm/dd hh24:mi:ss'),'yyyy/mm/dd hh24:mi:ss') as sign_date
from att_addrecordform a    
join  att_member b on b.member_id=a.member_id and b.account_id=-1771325801755686689
where a.is_onway=0 and to_char(a.record_datetime/(1000*60*60*24)+to_date('1970-01-01 08:00:00','yyyy/mm/dd hh24:mi:ss'),'yyyy-mm-dd')= :signdate
order by sign_date desc" ;
    
    let statem = conn.statement(sql).build();
    let mut stmt = match statem {
        Ok(st) => st,
        Err(error) => panic!("stmt problem:{}", error),
    };

    //let rows = stmt.query(&[&"袁小文"]).unwrap();
    let rows = stmt.query(&[&signdate]).unwrap();
    println!("rows: {:?}", rows);
    /* let row_result = match rows {
        Ok(rw) => rw,
        Err(error) => panic!("row_result problem :{}", error);
     }
    */
 
    let mut row_vec: Vec<OaSigndate> = Vec::new();
    
    for row_result in rows {
        let mut memb: OaSigndate = OaSigndate{member_code:String::from("1"), member_name: String::from("test"), sign_date: String::from("2024/05/27 08:00:00")};
        let mut member_code : String = String::from("1");
        let mut member_name :String = String::from("test");
        let mut sign_date : String = String::from("2024/05/27 08:00:00");
        for (idx, val) in row_result.expect("row_result error:").sql_values().iter().enumerate() {
            
            if idx !=0 {
                print!(",");
            }
            print!("{}", val);            
            
            if idx ==0 {           
                //id = val.get().expect("val id error");
                member_code =  val.to_string();
            }

            if idx ==1 {
                member_name = val.to_string();
            }

            if idx == 2{
                sign_date = val.to_string();
            }
        }
        memb =OaSigndate{member_code: member_code, member_name: member_name, sign_date: sign_date};
        row_vec.push(memb);
        println!();
    }
    

    //println!("row_vec: {:?}", row_vec);

   /* let member = OrgMember{
        id : member_id,
        name : String::from("袁小文"),
     };
      Json(member)
      */
    Json(row_vec)

    //Html("<h1>Hello,world!</h1>")
}

/* async fn findMember(member_id: u64) -> OrgMember {
    OrgMember {
        id: member_id,
        name: String::from("袁小文"),
    }
}
*/

#[tokio::main]
async  fn main() {
    //let app = Router::new().route("/c1pro_members/:member_id",get(getOrgMember));
    let app = Router::new().route("/oasigndate/:signdate",get(getOaSigndate));

    let listener = tokio::net::TcpListener::bind("188.188.1.26:8080")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
