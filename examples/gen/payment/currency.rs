//! WARNING: This file is generated, derived from table payment.currency, DO NOT EDIT

use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use gen::bazaar::Product;
use gen::payment::Country;
use gen::payment::ExchangeRate;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Table;
use rustorm::table::Foreign;
use rustc_serialize::json::ToJson;
use rustc_serialize::json::Json;



#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Currency {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub currency_id: Uuid,
    /// which country uses this currency
    /// db data type: uuid
    pub country_id: Option<Uuid>,
    /// db data type: character varying
    pub symbol: Option<String>,
    /// db data type: character varying
    pub unicode: Option<String>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    /// db data type: boolean
    pub active: bool,
    /// --inherited-- 
    /// db data type: uuid
    pub client_id: Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    /// db data type: timestamp with time zone
    pub created: DateTime<UTC>,
    /// --inherited-- 
    /// db data type: uuid
    pub created_by: Option<Uuid>,
    /// --inherited-- 
    /// db data type: character varying
    pub description: Option<String>,
    /// --inherited-- 
    /// db data type: text
    pub help: Option<String>,
    /// --inherited-- 
    /// db data type: character varying
    pub name: Option<String>,
    /// --inherited-- 
    /// db data type: uuid
    pub organization_id: Option<Uuid>,
    /// --inherited-- 
    /// db data type: numeric
    pub priority: Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    /// db data type: timestamp with time zone
    pub updated: DateTime<UTC>,
    /// --inherited-- 
    /// db data type: uuid
    pub updated_by: Option<Uuid>,

    /// has one
    pub country: Option<Country>,
    /// has many
    pub exchange_rate: Vec<ExchangeRate>,
    /// has many
    pub product: Vec<Product>,
}



impl IsDao for Currency{
    fn from_dao(dao:&Dao)->Self{
        Currency{
            organization_id: dao.get_opt("organization_id"),
            client_id: dao.get_opt("client_id"),
            created: dao.get("created"),
            created_by: dao.get_opt("created_by"),
            updated: dao.get("updated"),
            updated_by: dao.get_opt("updated_by"),
            priority: dao.get_opt("priority"),
            name: dao.get_opt("name"),
            description: dao.get_opt("description"),
            help: dao.get_opt("help"),
            active: dao.get("active"),
            currency_id: dao.get("currency_id"),
            country_id: dao.get_opt("country_id"),
            symbol: dao.get_opt("symbol"),
            unicode: dao.get_opt("unicode"),
            country: None,
            exchange_rate: vec![],
            product: vec![],
        }
    }

    fn to_dao(&self)->Dao{
        let mut dao = Dao::new();
        match self.organization_id{
            Some(ref _value) => dao.set("organization_id", _value),
            None => dao.set_null("organization_id")
        }
        match self.client_id{
            Some(ref _value) => dao.set("client_id", _value),
            None => dao.set_null("client_id")
        }
        dao.set("created", &self.created);
        match self.created_by{
            Some(ref _value) => dao.set("created_by", _value),
            None => dao.set_null("created_by")
        }
        dao.set("updated", &self.updated);
        match self.updated_by{
            Some(ref _value) => dao.set("updated_by", _value),
            None => dao.set_null("updated_by")
        }
        match self.priority{
            Some(ref _value) => dao.set("priority", _value),
            None => dao.set_null("priority")
        }
        match self.name{
            Some(ref _value) => dao.set("name", _value),
            None => dao.set_null("name")
        }
        match self.description{
            Some(ref _value) => dao.set("description", _value),
            None => dao.set_null("description")
        }
        match self.help{
            Some(ref _value) => dao.set("help", _value),
            None => dao.set_null("help")
        }
        dao.set("active", &self.active);
        dao.set("currency_id", &self.currency_id);
        match self.country_id{
            Some(ref _value) => dao.set("country_id", _value),
            None => dao.set_null("country_id")
        }
        match self.symbol{
            Some(ref _value) => dao.set("symbol", _value),
            None => dao.set_null("symbol")
        }
        match self.unicode{
            Some(ref _value) => dao.set("unicode", _value),
            None => dao.set_null("unicode")
        }
        dao
    }
}

impl ToJson for Currency{

    fn to_json(&self)->Json{
        self.to_dao().to_json()
    }
}

impl IsTable for Currency{

    fn table()->Table{
    
        Table{
            schema:"payment".to_string(),
            name:"currency".to_string(),
            parent_table:Some("record".to_string()),
            sub_table:vec![],
            comment:None,
            columns:
            vec![
                Column{
                    name:"organization_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"client_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"created".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:true, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"created_by".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"updated".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:true, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"updated_by".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"priority".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"name".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"description".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"help".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"text".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"active".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:true, 
                    default:Some("true".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"currency_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:true, is_unique:false, not_null:true, is_inherited:false, 
                    default:Some("uuid_generate_v4()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"country_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:Some("which country uses this currency".to_string()),
                    foreign:Some(
                        Foreign{
                            schema:"payment".to_string(),
                            table:"country".to_string(),
                            column:"country_id".to_string(),
                        }),
                },
                Column{
                    name:"symbol".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"unicode".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
            ],
        }
    }
}
// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static organization_id: &'static str = "currency.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "currency.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "currency.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "currency.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "currency.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "currency.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "currency.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "currency.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static description: &'static str = "currency.description";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static help: &'static str = "currency.help";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static active: &'static str = "currency.active";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static currency_id: &'static str = "currency.currency_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static country_id: &'static str = "currency.country_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static symbol: &'static str = "currency.symbol";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static unicode: &'static str = "currency.unicode";
