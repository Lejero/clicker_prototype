use druid::im::{HashMap, OrdMap};
use druid::widget::ListIter;
use druid::*;

use crate::game::resource::Resource;
use crate::ui::events::{UPDATE_RESOURCE, UPDATE_RESOURCES};
use std::convert::TryInto;
use std::fmt;
// use std::fmt::{Debug, Display};
use std::ops::{Index, IndexMut};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
use std::vec::*;

pub enum ResourceNames {
    IronOre,
    CopperOre,
    TinOre,
    NickelOre,
    Coal,
    IronPlate,
    CopperPlate,
    TinPlate,
    NickelPlate,
    Steel,
    Other(String),
}

#[derive(Clone, Lens, Data, PartialEq)]
pub struct Resources {
    // pub ls: im::HashMap<String, Resource>,
    pub ls: OMT<String, Resource>,
    //pub ls: im::Vector<Resource>,
}

impl Resources {
    pub fn new() -> Self {
        Resources {
            ls: OMT { m: im::OrdMap::new() },
        }
    }

    pub fn add_resource(&mut self, res: Resource) {
        self.ls.m.insert(res.name.clone(), res);
    }
}

// #[derive(Clone, Data)]
// type TT = (String, Resource);

// type<A: Data, B: Data> HMT = im::HashMap<A, B>;

#[derive(Clone, Data, PartialEq)]
pub struct HMT<A, B>
where
    A: Data + std::hash::Hash + std::cmp::Eq,
    B: Data,
{
    pub m: im::HashMap<A, B>,
}
#[derive(Clone, Data, PartialEq)]
pub struct OMT<K, V>
where
    K: Data + std::cmp::Ord,
    V: Data,
{
    pub m: im::OrdMap<K, V>,
}
// struct HMV<'a, A, B>
// where
//     A: Data + std::hash::Hash + std::cmp::Eq,
//     B: Data,
// {
//     pub values: im::hashmap::Values<'a, A, B>,
// }

// #[derive(Clone, Data)]
// struct HM(im::HashMap<String, Resource>);
// impl ListIter<(String, Resource)> for HM {
//     fn for_each(&self, cb: impl FnMut(&(String, Resource), usize)) {
//         for (i, item) in self.0.iter().enumerate() {
//             let ret = (item.0.to_owned(), item.1.to_owned());
//             cb(&ret, i);
//         }
//     }

//     fn for_each_mut(&mut self, cb: impl FnMut(&mut (String, Resource), usize)) {
//         for (i, item) in self.0.iter_mut().enumerate() {
//             let mut ret = (item.0.clone(), item.1.clone());
//             cb(&mut ret, i);

//             if !self.0.same(&ret.0) {
//                 self.0 = ret.0;
//             }
//             if !item.same(&ret.1) {
//                 *item = ret.1;
//             }
//         }
//     }

//     fn data_len(&self) -> usize {
//         self.0.len()
//     }
// }

// impl<'a, A, B> ListIter<(A, B)> for HMV<'a, A, B>
// where
//     A: Data + std::hash::Hash + std::cmp::Eq,
//     B: Data,
// {
//     fn for_each(&self, mut cb: impl FnMut(&(A, B), usize)) {
//         for (i, item) in self.m.iter().enumerate() {
//             let ret = (item.0.to_owned(), item.1.to_owned());
//             cb(&ret, i);
//         }
//     }

//     fn for_each_mut(&mut self, mut cb: impl FnMut(&mut (A, B), usize)) {
//         //We don't have an iter_mut which makes this trickier
//         //We're cloning m self.m because iter() immutably borrows it and we'd like to use it immutably later.Eq
//         //This constitutes a lot of overhead.
//         for (i, item) in self.m.clone().iter().enumerate() {
//             let mut ret = (item.0.clone(), item.1.clone());
//             cb(&mut ret, i);

//             // if !self.m.same(&ret) {
//             //     self.m = ret;
//             // }

//             //One possibility is if the key changes, this seems both rarer and trickier.
//             if !item.0.same(&ret.0) {
//                 if !self.m.contains_key(&item.0) {} //m will always contain this key.
//                 if !self.m.contains_key(&ret.0) {} //This is a problem
//             }
//             if !item.1.same(&ret.1) {
//                 self.m[&ret.0] = ret.1;
//             }
//         }
//     }

//     fn data_len(&self) -> usize {
//         self.m.len()
//     }
// }

impl<K, V> ListIter<V> for HMT<K, V>
where
    K: Data + std::hash::Hash + std::cmp::Eq,
    V: Data,
{
    fn for_each(&self, mut cb: impl FnMut(&V, usize)) {
        for (i, item) in self.m.iter().enumerate() {
            let ret = (item.0.to_owned(), item.1.to_owned());
            cb(&ret.1, i);
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut V, usize)) {
        //We don't have an iter_mut which makes this trickier
        //We're cloning m self.m because iter() immutably borrows it and we'd like to use it immutably later.Eq
        //This constitutes a lot of overhead.
        for (i, item) in self.m.clone().iter().enumerate() {
            let mut ret = (item.0.clone(), item.1.clone());
            cb(&mut ret.1, i);

            if !item.1.same(&ret.1) {
                self.m[&ret.0] = ret.1;
            }
        }
    }

    fn data_len(&self) -> usize {
        self.m.len()
    }
}

impl<K, V> ListIter<V> for OMT<K, V>
where
    K: Data + std::cmp::Ord,
    V: Data,
{
    fn for_each(&self, mut cb: impl FnMut(&V, usize)) {
        for (i, item) in self.m.iter().enumerate() {
            let ret = (item.0.to_owned(), item.1.to_owned());
            cb(&ret.1, i);
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut V, usize)) {
        //We don't have an iter_mut which makes this trickier
        //We're cloning m self.m because iter() immutably borrows it and we'd like to use it immutably later.Eq
        //This constitutes a lot of overhead.
        for (i, item) in self.m.clone().iter().enumerate() {
            let mut ret = (item.0.clone(), item.1.clone());
            cb(&mut ret.1, i);

            if !item.1.same(&ret.1) {
                self.m[&ret.0] = ret.1;
            }
        }
    }

    fn data_len(&self) -> usize {
        self.m.len()
    }
}

// impl<A, B> ListIter<(A, B)> for HMT<A, B>
// where
//     A: Data + std::hash::Hash + std::cmp::Eq,
//     B: Data,
// {
//     fn for_each(&self, mut cb: impl FnMut(&(A, B), usize)) {
//         for (i, item) in self.m.iter().enumerate() {
//             let ret = (item.0.to_owned(), item.1.to_owned());
//             cb(&ret, i);
//         }
//     }

//     fn for_each_mut(&mut self, mut cb: impl FnMut(&mut (A, B), usize)) {
//         //We don't have an iter_mut which makes this trickier
//         //We're cloning m self.m because iter() immutably borrows it and we'd like to use it immutably later.Eq
//         //This constitutes a lot of overhead.
//         for (i, item) in self.m.clone().iter().enumerate() {
//             let mut ret = (item.0.clone(), item.1.clone());
//             cb(&mut ret, i);

//             // if !self.m.same(&ret) {
//             //     self.m = ret;
//             // }

//             //One possibility is if the key changes, this seems both rarer and trickier.
//             if !item.0.same(&ret.0) {
//                 if !self.m.contains_key(&item.0) {} //m will always contain this key.
//                 if !self.m.contains_key(&ret.0) {} //This is a problem
//             }
//             if !item.1.same(&ret.1) {
//                 self.m[&ret.0] = ret.1;
//             }
//         }
//     }

//     fn data_len(&self) -> usize {
//         self.m.len()
//     }
// }

// pub struct ResourceLens;

// impl Lens<Resources, im::Vector<Resource>> for ResourceLens {
//     fn with<R, F: FnOnce(&im::Vector<Resource>) -> R>(&self, data: &Resources, f: F) -> R {
//         let hmap = data.clone();

//         let mut vect = im::Vector::<Resource>::new();

//         for v in hmap.ls.iter().map(|x| x.1) {
//             vect.push_back(v.clone());
//         }

//         f(&vect)
//     }

//     fn with_mut<R, F: FnOnce(&mut im::Vector<Resource>) -> R>(&self, data: &mut Resources, f: F) -> R {
//         let hmap = data.ls.clone();

//         //if changed {
//         let mut vect = im::Vector::<Resource>::new();

//         for v in hmap.iter().map(|x| x.1) {
//             vect.push_back(v.clone());
//         }

//         let result = f(&mut vect);

//         let mut ovect = im::Vector::<Resource>::new();
//         for v in hmap.iter().map(|x| x.1) {
//             ovect.push_back(v.clone());
//         }

//         let changed = !vect.same(&ovect);
//         if changed {
//             for ind in vect.iter() {
//                 if data.ls.contains_key(&ind.name) {
//                     data.ls[&ind.name] = ind.clone();
//                 } else {
//                     data.add_resource(ind.clone());
//                 }
//             }
//         }

//         result
//     }
// }
