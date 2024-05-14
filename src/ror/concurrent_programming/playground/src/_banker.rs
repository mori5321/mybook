// use std::sync::{Arc, Mutex};
// use std::thread;
//
// struct Resource<const NRES: usize, const NTH: usize> {
//     available: [usize; NRES], // 利用可能なリソース
//     allocation: [[usize; NRES]; NTH], // スレッドiが確保中のリソース
//     max: [[usize; NRES]; NTH], // スレッドiが確保したい最大リソース
// }
//
// impl <const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
//     fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
//         Self {
//             available,
//             allocation: [[0; NRES]; NTH],
//             max,
//         }
//     }
//
//     fn is_safe(&self) -> bool {
//         let mut finish = [false; NTH];
//         let mut work = self.available.clone(); // 利用可能なリソース
//
//         loop {
//             // すべてのスレッドiとリソースjにおいて、
//             // finish[i] == false かつ work[j] >= (self.max[i][j] - self.allocation[i][j])
//             // を満たすようなスレッドを見つける
//             //
//             // すなわち、スレッドiが完了していないかつ、
//             // スレッドiがこれから確保したいリソースが利用可能なリソースを超えていないような
//             // スレッドiを見つける
//             let mut found = false;
//             let mut num_true = 0;
//             for (i, alc) in self.allocation.iter().enumerate() {
//                 if finish[i] {
//                     num_true += 1;
//                     continue;
//                 }
//
//                 let need = self.max[i].iter().zip(alc).map(|(m, a)| m - a);
//                 let is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);
//                 if is_avail {
//                     // スレッドiがリソース確保可能ならば
//                     found = true;
//                     finish[i] = true;
//                     for (w, a) in work.iter_mut().zip(alc) {
//                         *w += *a;
//                     }
//                     break;
//                 }
//
//             }
//
//             if num_true == NTH {
//                 // すべてのスレッドがリソース確保可能ならば安全であるとする
//                 return true;
//             }
//
//             if !found { // リソースを確保できないスレッドがある
//                 break;
//             }
//         }    false
//     }
//
//     // id番目のスレッドがresourceを1つ取得する処理
//     fn take(&mut self, id: usize, resource: usize) -> bool {
//         // スレッド番号が範囲外、リソースサイズが範囲外、
//         // 利用可能なリソースが0、スレッドiが確保中のリソースがスレッドiが確保したい最大リソースと等しい
//         // 場合はリソース確保に失敗させる
//         if id > NTH || resource >= NRES || self.available[resource] == 0 || self.max[id][resource] == self.allocation[id][resource] {
//             return false;
//         }
//
//         self.allocation[id][resource] += 1;
//         self.available[resource] -=1;
//
//         if self.is_safe() {
//             // 安全な状態(=すべてのスレッドに対してリソースを確保する余裕がある)
//             // ならばリソース確保に成功させる
//             true 
//         } else {
//             // 危険な場合ならば、リソース確保に失敗させる
//             // そして状態を復元する
//             self.allocation[id][resource] -= 1;
//             self.available[resource] += 1;
//             false
//         }
//     }
//
//     // id番目のスレッドがresourceを1つ解放する処理
//     fn release(&mut self, id: usize, resource: usize) -> () {
//         if id > NTH || resource >= NRES || self.allocation[id][resource] == 0 {
//             return;
//         }
//
//         self.allocation[id][resource] -= 1;
//         self.available[resource] += 1;
//     }
// }
//
//
// #[derive(Clone)]
// pub struct Banker<const NRES: usize, const NTH: usize> {
//     resource: Arc<Mutex<Resource<NRES, NTH>>>,
// }
//
// impl <const NRES: usize, const NTH: usize> Banker<NRES, NTH> {
//     pub fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
//         Self {
//             resource: Arc::new(Mutex::new(Resource::new(available, max))),
//         }
//     }
//
//     pub fn take(&self, id: usize, resource: usize) -> bool {
//         let mut r = self.resource.lock().unwrap();
//         r.take(id, resource)
//     }
//
//     pub fn release(&self, id: usize, resource: usize) {
//         let mut r = self.resource.lock().unwrap();
//         r.release(id, resource);
//     }
// }
//
// const NUM_LOOP: usize = 100000;
//
// fn main() {
//     let banker = Banker::<2, 2>::new([1, 1], [[1, 1], [1, 1]]);
//     let banker0 = banker.clone();
//
//     let philosopher0 = thread::spawn(move || {
//         for _ in 0..NUM_LOOP {
//             // banker0が箸0と箸1を確保
//             while !banker0.take(0, 0) {}
//             while !banker0.take(0, 1) {}
//
//             println!("0: eating");
//
//             // banker0が箸0と箸1を解放
//             banker0.release(0, 0);
//             banker0.release(0, 1);
//         }
//     });
//
//     let philosopher1 = thread::spawn(move || {
//         for _ in 0..NUM_LOOP {
//             // bankerが箸1と箸0を確保
//             while !banker.take(1, 1) {}
//             while !banker.take(1, 0) {}
//
//             println!("1: eating");
//
//             // bankerが箸1と箸0を解放
//             banker.release(1, 1);
//             banker.release(1, 0);
//         }
//     });
//    
//     philosopher0.join().unwrap();
//     philosopher1.join().unwrap();
// }
