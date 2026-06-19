// [3822\. Design Order Management System 🔒](https://leetcode.com/problems/design-order-management-system)
// ========================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// Description
// -----------

// You are asked to design a simple order management system for a trading platform.

// Each order is associated with an `orderId`, an `orderType` (`"buy"` or `"sell"`), and a `price`.

// An order is considered **active** unless it is canceled.

// Implement the `OrderManagementSystem` class:

// *   `OrderManagementSystem()`: Initializes the order management system.
// *   `void addOrder(int orderId, string orderType, int price)`: Adds a new **active** order with the given attributes. It is **guaranteed** that `orderId` is unique.
// *   `void modifyOrder(int orderId, int newPrice)`: Modifies the **price** of an existing order. It is **guaranteed** that the order exists and is _active_.
// *   `void cancelOrder(int orderId)`: Cancels an existing order. It is **guaranteed** that the order exists and is _active_.
// *   `vector<int> getOrdersAtPrice(string orderType, int price)`: Returns the `orderId`s of all **active** orders that match the given `orderType` and `price`. If no such orders exist, return an empty list.

// **Note:** The order of returned `orderId`s does not matter.

// **Example 1:**

// **Input:**
// \["OrderManagementSystem", "addOrder", "addOrder", "addOrder", "getOrdersAtPrice", "modifyOrder", "modifyOrder", "getOrdersAtPrice", "cancelOrder", "cancelOrder", "getOrdersAtPrice"\]
// \[\[\], \[1, "buy", 1\], \[2, "buy", 1\], \[3, "sell", 2\], \["buy", 1\], \[1, 3\], \[2, 1\], \["buy", 1\], \[3\], \[2\], \["buy", 1\]\]

// **Output:**
// \[null, null, null, null, \[2, 1\], null, null, \[2\], null, null, \[\]\]

// **Explanation**

// OrderManagementSystem orderManagementSystem = new OrderManagementSystem();
// orderManagementSystem.addOrder(1, "buy", 1); // A buy order with ID 1 is added at price 1.
// orderManagementSystem.addOrder(2, "buy", 1); // A buy order with ID 2 is added at price 1.
// orderManagementSystem.addOrder(3, "sell", 2); // A sell order with ID 3 is added at price 2.
// orderManagementSystem.getOrdersAtPrice("buy", 1); // Both buy orders (IDs 1 and 2) are active at price 1, so the result is `[2, 1]`.
// orderManagementSystem.modifyOrder(1, 3); // Order 1 is updated: its price becomes 3.
// orderManagementSystem.modifyOrder(2, 1); // Order 2 is updated, but its price remains 1.
// orderManagementSystem.getOrdersAtPrice("buy", 1); // Only order 2 is still an active buy order at price 1, so the result is `[2]`.
// orderManagementSystem.cancelOrder(3); // The sell order with ID 3 is canceled and removed from active orders.
// orderManagementSystem.cancelOrder(2); // The buy order with ID 2 is canceled and removed from active orders.
// orderManagementSystem.getOrdersAtPrice("buy", 1); // There are no active buy orders left at price 1, so the result is `[]`.

// **Constraints:**

// *   `1 <= orderId <= 2000`
// *   `orderId` is **unique** across all orders.
// *   `orderType` is either `"buy"` or `"sell"`.
// *   `1 <= price <= 109`
// *   The total number of calls to `addOrder`, `modifyOrder`, `cancelOrder`, and `getOrdersAtPrice` does not exceed 2000.
// *   For `modifyOrder` and `cancelOrder`, the specified `orderId` is **guaranteed** to exist and be _active_.

use std::collections::HashMap;

struct OrderManagementSystem {
    orders: HashMap<i32, (String, i32)>,
    t: HashMap<(String, i32), Vec<i32>>,
}

impl OrderManagementSystem {
    fn new() -> Self {
        Self {
            orders: HashMap::new(),
            t: HashMap::new(),
        }
    }

    fn add_order(&mut self, order_id: i32, order_type: String, price: i32) {
        self.orders.insert(order_id, (order_type.clone(), price));
        self.t
            .entry((order_type, price))
            .or_insert_with(Vec::new)
            .push(order_id);
    }

    fn modify_order(&mut self, order_id: i32, new_price: i32) {
        if let Some((order_type, old_price)) = self.orders.get(&order_id).cloned() {
            self.orders
                .insert(order_id, (order_type.clone(), new_price));

            if let Some(v) = self.t.get_mut(&(order_type.clone(), old_price)) {
                if let Some(pos) = v.iter().position(|&x| x == order_id) {
                    v.remove(pos);
                }
            }

            self.t
                .entry((order_type, new_price))
                .or_insert_with(Vec::new)
                .push(order_id);
        }
    }

    fn cancel_order(&mut self, order_id: i32) {
        if let Some((order_type, price)) = self.orders.remove(&order_id) {
            if let Some(v) = self.t.get_mut(&(order_type, price)) {
                if let Some(pos) = v.iter().position(|&x| x == order_id) {
                    v.remove(pos);
                }
            }
        }
    }

    fn get_orders_at_price(&self, order_type: String, price: i32) -> Vec<i32> {
        self.t
            .get(&(order_type, price))
            .cloned()
            .unwrap_or_default()
    }
}

// /**
//  * Your OrderManagementSystem object will be instantiated and called as such:
//  * let obj = OrderManagementSystem::new();
//  * obj.add_order(orderId, orderType, price);
//  * obj.modify_order(orderId, newPrice);
//  * obj.cancel_order(orderId);
//  * let ret_4: Vec<i32> = obj.get_orders_at_price(orderType, price);
//  */
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_order_management_system_1() {
        let mut order_management_system = OrderManagementSystem::new();
        order_management_system.add_order(1, "buy".to_owned(), 1);
        order_management_system.add_order(2, "buy".to_owned(), 1);
        order_management_system.add_order(3, "sell".to_owned(), 2);
        assert_eq!(
            vec![2, 1],
            order_management_system.get_orders_at_price("buy".to_owned(), 1)
        );
        order_management_system.modify_order(1, 3);
        order_management_system.modify_order(2, 1);
        assert_eq!(
            vec![2],
            order_management_system.get_orders_at_price("buy".to_owned(), 1)
        );
        order_management_system.cancel_order(3);
        order_management_system.cancel_order(2);
        assert_eq!(
            order_management_system.get_orders_at_price("buy".to_owned(), 1),
            vec![]
        );
    }
}
