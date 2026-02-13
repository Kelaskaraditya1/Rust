/*
 Traits: Traits are similar to interfaces in Java.  

 trait is similar to a interface and struct are classes who implement them.

Syntax:
 trait <name>{
 fn method(parameter:type)->type;
 }

 struct <name>{
 p1:type,
 p2:type,
 ...
 }

 impl <trait name> for <struct name>{
    fn method(parameter:type)->{
    implementation...
    }
 }

 if there are multiple structs and we have to implement the same logic between them so we can implement them with a common trait and pass the type of trait in the function parameter
 so that while function call we can pass any of the struct type as the parameter.

 fn name(impl <trait name>){
 implementation ...
 }

 name(param:type1)
 name(param:type2)

 boh type1 and type2 are struct type which implement a common trait.

*/
#![allow(dead_code,unused_variables)]
trait Summary {
    fn summerize(&self)->String;
}

#[derive(Debug)]
struct NewsLetter{
    headline:String,
    location:String,
    author:String,
    content:String
}

#[derive(Debug)]
struct Tweet{

    username:String,
    content:String,
    reply:String,
    re_tweet:String

}

impl Summary for Tweet {
    fn summerize(&self)->String {
        let message = format!("The news is {} and it is published by {}",self.content,self.username);
        message
    }
}

impl Summary for NewsLetter{
       fn summerize(&self)->String {
        let message = format!("The news is {} and it is published by {}",self.content,self.author);
        message
    } 
}


fn news_aggregator(summary: &impl Summary){
    println!("There is a new news in the market");
    println!("{}",summary.summerize());
}

fn main(){
    println!("Traits");

    let news_letter = NewsLetter{
        headline:String::from("Aaj ki taja khabar"),
        location:String::from("Bhandup,Mumbai"),
        author:String::from("Aditya Kelaskar"),
        content:String::from("Kon hai Hamza Ali Mazari !!")
    };

        let tweet =Tweet{
        content:String::from("Aaj ki taja khabar"),
        re_tweet:String::from("Bhandup,Mumbai"),
        username:String::from("Aditya Kelaskar"),
        reply:String::from("Kon hai Hamza Ali Mazari !!")
    };

    println!("{:?}",news_aggregator(&news_letter));
    println!("{:?}",news_aggregator(&tweet));


}