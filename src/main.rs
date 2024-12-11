use eframe::egui;
use eframe::NativeOptions;
use fastrand;
use std::{cmp::Ordering,time::Instant};

fn main() {
    let native_options=NativeOptions::default();

    let my_app=MyApp::new();

    eframe::run_native("Guessing Number Game"
                    , native_options
                    , Box::new(|_| Ok(Box::new(my_app)))).unwrap();
}

fn get_answer(range:&Range) -> String {
    fastrand::u32(range.0..=range.1).to_string()
}

struct Range(u32,u32);

enum AppState {
    StartWindow,
    MainWindow,
    SetRangeWindow,
    GameWindow,
    VictoryWindow,
    FailWindow(Option<Ordering>),
    HelpWindow,
}

struct MyApp {
    app_state:AppState,
    answer:String,
    input:String,
    counter:u8,
    range:Range,

    start_window_isok:Option<Instant>,
}

impl MyApp {
    fn new() -> Self {
        println!("MyApp newed");

        let range=Range(1,100);

        MyApp { 
            app_state: AppState::StartWindow,
            answer: get_answer(&range),
            input:String::new(),
            counter:5_u8,
            range,
            start_window_isok:None,
        }
    }

    fn less_counter(&mut self) {
        println!("self.counter = {}",self.counter);

        if self.counter > 0 {
            self.counter-=1;
        }
    }

    fn init(&mut self) {
        self.answer=get_answer(&self.range);
        self.counter=5_u8;

        println!("init... answer = {}",self.answer);
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.app_state {
            AppState::StartWindow => {
                // 标题
                egui::CentralPanel::default().show(ctx, |ui| {
                    let layout=egui::Layout::centered_and_justified(egui::Direction::TopDown);
                    ui.with_layout(layout, |ui| {
                        ui.heading("Guessing Number Game");
                    });
                });

                // 实现延迟2秒再跳转到主菜单的功能
                match self.start_window_isok {
                    None => self.start_window_isok=Some(Instant::now()),
                    Some(start_time) => {
                        let elapsed_time=start_time.elapsed();

                        if elapsed_time.as_secs() >= 2 {
                            self.app_state=AppState::MainWindow;
                            self.start_window_isok=None;
                        }
                    }
                }
            }
            AppState::MainWindow => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.add_space(100.0);
                    // ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown),|ui| {
                    //     ui.heading("Guessing Number Game");
                    //     ui.label("You need to guess a number which is in [1,100]");
                    //     ui.label("You can try only five times ! ! !");
                    // });
                    let top_down_layout = egui::Layout::top_down(egui::Align::Center);
                    ui.with_layout(top_down_layout, |ui| {
                        ui.heading("Guessing Number Game");
                        ui.add_space(30.0);

                        let context=format!("You need to guess a number which is in [{},{}]",self.range.0,self.range.1);
                        ui.label(context);
                        ui.add_space(30.0);
                        ui.label("You can try only five times!!!");
                        ui.add_space(30.0);

                        if ui.button("START").clicked() {
                            self.init();
                            self.app_state=AppState::GameWindow;
                        }

                        ui.add_space(30.0);

                        if ui.button("set range").clicked() {
                            self.app_state=AppState::SetRangeWindow;
                        }
                    });

                    let button_layout=egui::Layout::right_to_left(egui::Align::Max);
                    ui.with_layout(button_layout, |ui| {
        
                        if ui.button("editor").clicked() {
                            self.app_state=AppState::HelpWindow;
                        }
                    });
                });
            }

            AppState::SetRangeWindow => {
                egui::CentralPanel::default().show(ctx,|ui| {
                    let layout=egui::Layout::top_down(egui::Align::Center);
                    ui.with_layout(layout, |ui| {
                        ui.add_space(100.0);

                        ui.label("You can change the range!");

                        ui.add_space(50.0);

                        if ui.button("[1,100]").clicked() {
                            self.range=Range(1,100);
                            self.app_state=AppState::MainWindow;
                        }

                        ui.add_space(50.0);

                        if ui.button("[1,200]").clicked() {
                            self.range=Range(1,200);
                            self.app_state=AppState::MainWindow;
                        }

                        ui.add_space(50.0);

                        if ui.button("[25,75]").clicked() {
                            self.range=Range(25,75);
                            self.app_state=AppState::MainWindow;
                        }
                    });

                    // let layout=egui::Layout::right_to_left(egui::Align::Max);
                    // ui.with_layout(layout, |ui| {
                    //     if ui.button("Ok").clicked() {
                    //         self.app_state=AppState::MainWindow;
                    //     }
                    // });
                });
            }

            AppState::GameWindow => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    // self.init();
                    let mut input=String::new();

                    let layout=egui::Layout::top_down(egui::Align::Center);
                    ui.with_layout(layout, |ui| {
                        ui.add_space(150.0);
                        ui.heading("Now input you answer...");
                        ui.add_space(30.0);
                        let response=ui.text_edit_singleline(&mut input);

                        if response.changed() {
                            self.input.push_str(&input);
                        }

                        if response.lost_focus() {
                            println!("input = {}",self.input);
    
                            if let Ok(input_num) = self.input.trim().parse::<u32>() {
                                let right_answer=self.answer.trim().parse::<u32>().unwrap();
    
                                match input_num.cmp(&right_answer) {
                                    Ordering::Equal => {
                                        self.app_state=AppState::VictoryWindow;
                                    }
                                    Ordering::Greater => {
                                        self.less_counter();
                                        self.app_state=AppState::FailWindow(Some(Ordering::Greater));
                                    }
                                    Ordering::Less => {
                                        self.less_counter();
                                        self.app_state=AppState::FailWindow(Some(Ordering::Less));
                                    }
                                }
                            } else {
                                self.less_counter();
                                self.app_state=AppState::FailWindow(None);
                            }
                            self.input.clear();
                        } // lost_focus
                    });

                    
                }); // show
            }

            AppState::VictoryWindow => {
                let window=egui::Window::new("You win ! ! !");

                window.fixed_pos([200.0,200.0]).show(ctx, |ui| {
                    ui.label("You can enjoy this game again.");

                    if ui.button("Replay").clicked() {
                        self.init();
                        self.app_state=AppState::GameWindow;
                    }

                    if ui.button("Come back to the MENU").clicked() {
                        self.app_state=AppState::MainWindow;
                    }
                });

            }

            AppState::FailWindow(numstate) => {
                if self.counter>0 {

                    let title =match numstate {
                        Some(Ordering::Greater) => "Bigger...".to_string(),
                        Some(Ordering::Less) => "Smaller...".to_string(),
                        _ => "Please input an int...".to_string(),
                    };

                    let window=egui::Window::new(title);
                    window.fixed_pos([200.0,200.0]).show(ctx, |ui| {
                        ui.label(format!("But you can try {} times again",self.counter));

                        if ui.button("Retry").clicked() {
                            self.app_state = AppState::GameWindow;
                        }

                        if ui.button("Come back to the MENU").clicked() {
                            self.app_state = AppState::MainWindow;
                        }

                    });
                } else {
                    let window=egui::Window::new("You fail...");

                    window.fixed_pos([200.0,200.0]).show(ctx, |ui| {
                        ui.label(format!("The right answer is {}",self.answer));

                        if ui.button("Replay").clicked() {
                            self.init();
                            self.app_state=AppState::GameWindow;
                        }

                        if ui.button("Come back to the MENU").clicked() {
                            self.app_state=AppState::MainWindow;
                        }
                    });
                }
            }  // FailWindow

            AppState::HelpWindow => {
                egui::CentralPanel::default().show(ctx, |ui| {

                    let layout=egui::Layout::top_down(egui::Align::Center);
                    ui.with_layout(layout,|ui| {
                        ui.add_space(150.0);
                        ui.add(egui::Label::new("editor: Moore-ai"));
                        ui.add_space(50.0);
                        
                        if ui.button("MENU").clicked() {
                            self.app_state=AppState::MainWindow;
                        }
                    });
                });
            }
        }
    }
}