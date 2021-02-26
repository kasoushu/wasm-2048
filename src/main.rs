use yew::{prelude::*, services::console};
use yew::services::keyboard::{KeyboardService, KeyListenerHandle};
use yew::utils;
use KeyboardEvent;
use std::cmp;
use rand::Rng;
use yew::services::ConsoleService;

struct Grid{
    grid:[[i32;5];5],
    max_number:i32,
    size:i32,
}

impl Grid{
    pub fn new() ->Grid {
        let mut g = Grid{
            grid:[[0;5];5],
            max_number:0,
            size:0,
        };
        g.set_cell(4);
        g
    }
    pub fn get_nums(&self,i:usize,j:usize) -> i32{
        self.grid[i][j]
    }
    pub fn get_max(&self) -> i32{
        self.max_number
    }
    pub fn to_move(&mut self,d : usize){
        ConsoleService::info("set begin()\n\r");
        //1 left,2 right,3 up,4 down;
        match d {
            1 =>{
                self.move_left();
            }
            2 =>{
                self.move_right();
            }
            3 =>{
                self.move_up();
            }
            4=>{
                self.move_down();
            }
            _ =>{}
        };
        self.set_cell(2);
        ConsoleService::info("set finish!\n\r");
    }
    pub fn move_left(& mut self){
        let a=self.grid.len();
        for row in self.grid.iter_mut(){
            for k in 0..row.len()
            {
                for i in 1..row.len(){
                    if row[i-1]==0 {
                        row[i-1]=row[i];
                        row[i]=0;
                    }
                    if row[i]==row[i-1]{
                        row[i-1]=row[i-1]*2;
                        row[i]=0;
                    }
                }
            }
        }
        let mut cnt =0;
        for i in self.grid.iter(){
            for j in i.iter(){
                if *j!=0{
                    cnt+=1;
                }
            }
        }
        self.size=cnt;
    }
    pub fn move_right(& mut self){
        let a=self.grid.len();
        for row in self.grid.iter_mut(){
            for k in 0..row.len(){
                for i in (0..row.len()-1).rev(){
                    if row[i+1]==0 {
                        row[i+1]=row[i];
                        row[i]=0;
                    }
                    if row[i]==row[i+1]{
                        row[i+1]=row[i+1]*2;
                        row[i]=0;
                    }
                }
            }
        }
        let mut cnt =0;
        for i in self.grid.iter(){
            for j in i.iter(){
                if *j!=0{
                    cnt+=1;
                }
            }
        }
        self.size=cnt;
    }
    pub fn move_up(& mut self){
        let a=self.grid.len();
        for j in 0..a{
            for k in 0..a{
                for i in 1..a{
                    if self.grid[i-1][j]==0{
                        self.grid[i-1][j]=self.grid[i][j];
                        self.grid[i][j]=0;
                    }
                    if self.grid[i-1][j]==self.grid[i][j]{
                        self.grid[i-1][j]=self.grid[i][j]*2;
                        self.grid[i][j]=0;
                    }
                }
            }
        }
        let mut cnt =0;
        for i in self.grid.iter(){
            for j in i.iter(){
                if *j!=0{
                    cnt+=1;
                }
            }
        }
        self.size=cnt;
    }
    pub fn move_down(& mut self){
        let a=self.grid.len();
        for j in 0..a{
            for k in 0..a{
                for i in (0..a-1).rev(){
                    if self.grid[i+1][j]==0{
                        self.grid[i+1][j]=self.grid[i][j];
                        self.grid[i][j]=0;
                    }
                    if self.grid[i+1][j]==self.grid[i][j]{
                        self.grid[i+1][j]=self.grid[i][j]*2;
                        self.grid[i][j]=0;
                    }
                }
            }
        }
        let mut cnt =0;
        for i in self.grid.iter(){
            for j in i.iter(){
                if *j!=0{
                    cnt+=1;
                }
            }
        }
        self.size=cnt;
    }
    pub fn pgo(&self) -> String{
        let mut s="".to_string();
        for row in self.grid.iter(){
            for k in row.iter()
            {
                s=s + &format!("{} ",k);
            }
            s.push_str("\n\r")
        }
        s
    }
    pub fn pg(&self){
        for row in self.grid.iter(){
            for k in row.iter()
            {
                print!{"{} ",k};
            }
            println!();
        }
    }


    pub fn set_cell(& mut self,k:i32){
        let mut rng = rand::thread_rng();
        let n = cmp::min(k,25-self.size);
        self.size+=n;
        let mut cnt =0;
        while cnt<n{
            let even = rng.gen_range(0..10);
            if even%2==0 {
                let x = rng.gen_range(0..5);
                let y = rng.gen_range(0..5);
                if self.get_nums(x,y)!=0 {continue;}
                    self.grid[x][y] = 2;
                    cnt+=1;
            }else{
                let x = rng.gen_range(0..5);
                let y = rng.gen_range(0..5);
                if self.get_nums(x,y)!=0{continue;}
                    self.grid[x][y] = 4;
                    cnt+=1;
            }
        }
    }
}


enum Msg {
    AddOne,
    Click,
    KeyboardEvent(KeyboardEvent),
    OutString(String),
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    grid:Grid,
    de:String,
    listener:KeyListenerHandle,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let listener = KeyboardService::register_key_down(
            &utils::document(),
            link.callback(|e:KeyboardEvent| Msg::KeyboardEvent(e))
        );
        Self {
            link,
            value: 0,
            grid:Grid::new(),
            listener,
            de:"".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::Click =>{
                self.value=100;
                true
            }
            Msg::KeyboardEvent(e) => {
                self.value=99;
                match  e.key().as_str() {
                    "a" =>{
                        self.grid.to_move(3);
                        self.de= self.grid.pgo();
                        ConsoleService::info(&self.grid.pgo());
                    }
                    "d" =>{
                        ConsoleService::info("dddd");
                        self.grid.to_move(4);
                        self.de= self.grid.pgo();
                        ConsoleService::info(&self.grid.pgo());
                    }
                    "w" =>{
                        self.grid.to_move(1);
                        self.de= self.grid.pgo();
                        ConsoleService::info(&self.grid.pgo());
                    }
                    "s" =>{
                        self.grid.to_move(2);
                        self.de= self.grid.pgo();
                        ConsoleService::info(&self.grid.pgo());
                    }
                    "q" =>{
                        self.de= self.grid.pgo();
                        ConsoleService::info(&self.grid.pgo());
                    }
                    "r" =>{
                        ConsoleService::info("r!");
                    }
                    _ =>{
                        self.value=95;
                    }
                }
                true
            }
            _ =>{
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let cell = |(x,y)|{
            html! {
                <>
                {
                if self.grid.get_nums(x,y)==0{
                    html!{
                    <div class="cell-zero cell" >
                        {" "}
                    </div>
                    }
                }else{
                    html!{
                    <div class=format!("cell-{} cell",self.grid.get_nums(x,y)) >
                        {self.grid.get_nums(x,y)}
                    </div>
                    }
                }
                }
                </>
            }
        };
        let row =|x|{
            html!{
                  <div class="row">
                        {for ((0..5)).zip((0..5).map(|_|{x})).map(cell)}
                  </div>
                 }
        };
        html! {
            <div id="game-2048">
                // html!{
                <p>{"2048 game"}</p>
                    <div class="grid">
                    // <ul>
                    {
                        for (0..5).map(row)
                    }
                    // </ul>
                    </div>
                 // }
                 <p2 class="foot">{ " press w a s d to move up/left/down/right "}</p2>
                 <br />
                 <p2 class="foot">{ " but i hadn't set the function of win or lose...... "}</p2>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}