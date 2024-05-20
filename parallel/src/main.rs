// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   main.rs                                            :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: ajaidi <ajaidi@student.42.fr>              +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2024/05/19 23:57:39 by ajaidi            #+#    #+#             //
//   Updated: 2024/05/20 00:00:48 by ajaidi           ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use std::{thread, time::Duration};

fn main() {
    let th1 = thread::spawn(|| {
        for i in 0..5u8 {
            println!("first thread => {}.", i);
            thread::sleep(Duration::from_secs_f64(0.2));
        }
    });
    let th2 = thread::spawn(|| {
        for i in 0..5u8 {
            println!("second thread => {}.", i);
            thread::sleep(Duration::from_secs_f64(0.2));
        }
    });
    let th3 = thread::spawn(|| {
        for i in 0..5u8 {
            println!("first thread => {}.", i);
            thread::sleep(Duration::from_secs_f64(0.2));
        }
    });

    th1.join().unwrap();
}
