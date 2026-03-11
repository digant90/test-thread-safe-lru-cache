/*
 * Least Recently Used (LRU) cache.
 *
 *
 * Two core list operations:
 * - push_front: insert a node right after the dummy head (MRU position)
 * - delete_end: remove the node right before the dummy tail (LRU position)
 */

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/*
 * -------------------------------------------------------------------------
 * Doubly-linked list node
 * ------------------------------------------------------------------------- 
 */

type Link = Rc<RefCell<DLL>>;

struct DLL {
    key: i32,
    value: i32,
    prev: Option<Link>,
    next: Option<Link>,
}

impl DLL {
    fn new(key: i32, value: i32) -> Link {
        Rc::new(RefCell::new(DLL {
            key,
            value,
            prev: None,
            next: None,
        }))
    }
}

/*
 * -------------------------------------------------------------------------
 * LRU cache
 * -------------------------------------------------------------------------
 */

/*
 * A Least Recently Used (LRU) cache with O(1) get and put.
 */

 pub struct LruCache {
    capacity: usize,
    map: HashMap<i32, Link>,
    head: Link, /* dummy head sentinel */
    tail: Link, /* dummy tail sentinel */
}

impl LruCache {
    /*
     * Create a new cache that holds at most `capacity` items.
     */
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "LRU cache capacity must be > 0");

        let head = DLL::new(-1, -1);
        let tail = DLL::new(-1, -1);

        /* Link head <-> tail. */
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::clone(&head));

        Self {
            capacity,
            map: HashMap::with_capacity(capacity),
            head,
            tail,
        }
    }

    /* -- two core linked-list operations ---------------------------------- */

    fn push_front(&self, node: &Link) {
        /* Remove from current position if linked. */
        if node.borrow().prev.is_some() {
            let prev = node.borrow().prev.as_ref().map(Rc::clone).unwrap();
            let next = node.borrow().next.as_ref().map(Rc::clone).unwrap();
            prev.borrow_mut().next = Some(Rc::clone(&next));
            next.borrow_mut().prev = Some(Rc::clone(&prev));
        }

        /* Insert after head. */
        let head_next = self.head.borrow().next.as_ref().map(Rc::clone).unwrap();

        node.borrow_mut().prev = Some(Rc::clone(&self.head));
        node.borrow_mut().next = Some(Rc::clone(&head_next));
        self.head.borrow_mut().next = Some(Rc::clone(node));
        head_next.borrow_mut().prev = Some(Rc::clone(node));
    }

    fn delete_end(&self) -> i32 {
        let lru = self.tail.borrow().prev.as_ref().map(Rc::clone).unwrap();
        let prev = lru.borrow().prev.as_ref().map(Rc::clone).unwrap();

        prev.borrow_mut().next = Some(Rc::clone(&self.tail));
        self.tail.borrow_mut().prev = Some(Rc::clone(&prev));

        lru.borrow_mut().prev = None;
        lru.borrow_mut().next = None;

        let key = lru.borrow().key;
        key
    }

    /* -- public API ------------------------------------------------------- */

    /*
     * Retrieve a clone of the value associated with key.
     *
     * Accessing a key promotes it to the most-recently-used position.
     */
    pub fn get(&mut self, key: &i32) -> Option<i32> {
        let node = self.map.get(key).map(Rc::clone)?;
        self.push_front(&node);
        let value = node.borrow().value;
        Some(value)
    }

    /*
     * Insert or update a key-value pair.
     *
     * If the cache is at capacity and the key is new, the least-recently-used
     * entry is evicted.
     */
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key).map(Rc::clone) {

            /* Key exists – update value and move to front. */
            node.borrow_mut().value = value;
            self.push_front(&node);
        } else {

            if self.map.len() == self.capacity {
                let evict_key = self.delete_end();
                self.map.remove(&evict_key);
            }

            let node = DLL::new(key, value);
            self.map.insert(key, Rc::clone(&node));
            self.push_front(&node);
        }
    }

    /* Return the number of items currently in the cache. */
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /* Return true if the cache is empty. */
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
