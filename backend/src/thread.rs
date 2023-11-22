use std::{
  thread,
  thread::JoinHandle,
  time::Duration,
};
use serde_json::json;
use rand::Rng;
use serde::{Serialize, Deserialize};
use reqwest::{
  blocking::Client,
  header::CONTENT_TYPE
};
use unicode_segmentation::UnicodeSegmentation;


#[derive(Debug, Deserialize)]
struct ResError {
    error: String
}

#[derive(Debug, Deserialize)]
struct ResSuccess {
    order: Order
}

// A vector containing the orders made for a create orders request
#[derive(Serialize, Deserialize, Debug)]
pub struct OrdersList {
  pub orders: Vec<Order>
}

// A single order contained in a create orders request, the table_number is provided by the url path
#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
  pub id: String,
  pub item: String,
  #[serde(rename = "cookTime")]
  pub cook_time: i16,
}


// Will run various get, create, and delete requests
pub fn create_client(id: i16) -> JoinHandle<()> {
  // An array of "menu items" for the restaurant
  let item_choice: [&str; 10] = [
    "<CLIENT> tacos",
    "<CLIENT> hamburger",
    "<CLIENT> pizza",
    "<CLIENT> pasta",
    "<CLIENT> french fries",
    "<CLIENT> soup",
    "<CLIENT> salad",
    "<CLIENT> steak",
    "<CLIENT> curry",
    "<CLIENT> sushi"
  ];

  return thread::spawn(move || {
    let client = Client::new();
    loop {
      // How long the client should wait between requests
      let mut rand_rng = rand::thread_rng();
      let sleep_time = rand_rng.gen_range(5..15);
      thread::sleep(Duration::from_secs(sleep_time));

      // Get a list of orders from a random table
      let rand_table = rand_rng.gen_range(1..100);
      let response = client.get("http://localhost:8000/api/tables/".to_owned() + &rand_table.to_string()).send().expect("REASON");//.expect("REASON").value();
      
      if response.error_for_status_ref().is_err() {
        let response_error: ResError = serde_json::from_str(&response.text().unwrap()).unwrap();
        println!("");
        println!("Client #{id} failed to retrieve table orders list with status: {:?}", response_error.error);
        continue;
      }
      
      let list: OrdersList = serde_json::from_str(&response.text().unwrap()).unwrap();
      let client_orders: Vec<&Order> = list.orders.iter()
        .filter(|order| filter_list(&order.item))
        .collect();
      
      // If there's less than 10 orders created by a client, create 3 randomly generated orders.
      if client_orders.len() < 10 {
        let rand_item_one = rand_rng.gen_range(0..9);
        let rand_item_two = rand_rng.gen_range(0..9);
        let rand_item_three = rand_rng.gen_range(0..9);
        let body = json!({"orders": [
          {"item":item_choice[rand_item_one], "cook_time":rand_rng.gen_range(1..30)},
          {"item":item_choice[rand_item_two], "cook_time":rand_rng.gen_range(1..30)},
          {"item":item_choice[rand_item_three], "cook_time":rand_rng.gen_range(1..30)},
        ]
        }).to_string();

        let create_res = client.post("http://localhost:8000/api/tables/".to_owned() + &rand_table.to_string())
          .header(CONTENT_TYPE, "application/json")
          .body(body.clone())
          .send()
          .expect("REASON");
        
        let create_result = serde_json::from_str::<OrdersList>(&create_res.text().unwrap()).unwrap();

        println!("");
        println!("Client #{id} sent orders {:?} for table: {:?}", create_result.orders, rand_table);
        continue;
      }
      
      // If there's 10 or more orders created by a client at this table, delete a client's order.
      //  First select a client order and run a get request on it.
      let oldest_order = client_orders[0];
      let get_response = client.get("http://localhost:8000/api/orders/".to_owned() + &oldest_order.id).send().expect("REASON");
      let get_res: ResSuccess = serde_json::from_str(&get_response.text().unwrap()).unwrap();
      let get_id: String = get_res.order.id;

      //  Then delete the order with the resulting id
      let delete_response = client.delete("http://localhost:8000/api/orders/".to_owned() + &get_id).send().expect("REASON");
      
      if delete_response.status() == 204 {
        println!("");
        println!("Client #{id} deleted order {:?} for table: {:?}", oldest_order, rand_table);
      }
    }
  })
}

fn filter_list(item: &String) -> bool {
  let preface = "<CLIENT>";
  let substring = item.graphemes(true).into_iter().take(8).collect::<String>();
  substring.eq(preface)
}
