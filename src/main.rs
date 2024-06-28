use ferris_says;//远程包
use std::io::{stdout, BufReader, BufWriter}; //标准库包
use ferris_says::say;//远程包say方法
extern crate secure_password;//crates.io上的远程包https://crates.io/search?q=secure_password
use anyhow::Result;//crates.io上的anyhow包
use std::fmt;//标准库fmt包
use std::fmt::Display;//标准库fmt包Display函数
use bytes::{Bytes,Buf,BytesMut,BufMut}; //crates.io上的bytes包

//读取字符串并返回Result对象
fn get_cluster_info() -> Result<String> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map: String = serde_json::from_str(&config)?;//json字符串反序列化
    Ok(map) //读取成功
}

//结构中使用特性，支持复制，克隆 ，调试
#[derive(Debug,Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

//实现默认值接口
impl Default for Point{
    fn default() -> Self {
        println!("init struct default values");
        return Point{x:25.0,y:25.0};
    }
}

//泛型实现
impl<T: Into<f64>> From<(T, T)> for Point {
    fn from((x, y): (T, T)) -> Self {
        Point {
            x: x.into(),
            y: y.into(),
        }
    }
}

//类型自定义
type Vector = (f64,f64);

//属性别名实现
impl Point {
    const ORIGIN: Self = Point { x: 0.0, y: 0.0 };
}

//实现结构静态方法
impl Point {
    //为结构实现distance_from_origin方法
    fn distance_from_origin(&self) -> f64 {
        println!("x={},y={}",self.x,self.y);
        let p = Self::default();
        println!("x.powi==>{}",p.x.powi(2));
        println!("x.powi==>{}",p.y.powi(2));
        println!("(x.powi(2) + y.powi(2)).sqrt()===> {}",(p.x.powi(2) + p.y.powi(2)).sqrt());
        (p.x.powi(2) + p.y.powi(2)).sqrt()
    }
    pub fn new() -> Self{
        Point{x:128.0,y:128.0}
    }
}

//附加标准库实现访求到结构
impl fmt::Display for Point {
    //实现格式化输出接口方法fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "struct Point({}, {})", self.x, self.y)
    }
}

//字节测试
fn bytes_demo(){
    let mut b = BytesMut::new();//创建可变字节对象
    b.put(&b"hello,bytes"[..]);//添加可变长度字节数组
    b.put_f32(1.23);//添加浮点数
    b.put_int(123, 0);//添加整数
    println!("{:?}",b);//输出
}

fn main() {
    println!("Hello, world!");
    //测试字节
    bytes_demo();
    //测试固定长度字节
    let mut phone = BytesMut::with_capacity(11);
    phone.put(&b"13811112222"[..]);
    println!("===> Phone Number: {:?},the copy:{:?}",phone,phone.clone());

    //标准输出流操作
    let out = stdout();
    let msg = String::from("Hello Rust!");
    let len = msg.chars().count();//计算字符串长度
    //创建可写缓冲区
    let mut wr=BufWriter::new(out.lock());
    //向流中写入数据
    say(&msg,len,&mut wr).unwrap();

    //明文hash加密与验证测试
    let pwd =b"HelloRust";
    let hash = secure_password::hash(pwd).unwrap();
    let ok=secure_password::verify(pwd,&hash).unwrap();
    println!("{:?}",&hash);
    assert!(ok);

    //结构初始化
    let p = Point{x: 5.0,y: 5.0 };
    //结构别名使用
    let x = Point::ORIGIN.x;
    let y = Point::ORIGIN.y;
    println!("===> {}",p.distance_from_origin());
    println!("===> use format out: {}",p);

    //结构扩展函数使用
    println!("===> impl new fn for call: ",);
    let p1 = Point::new();
    println!("===>p1.x={},p1.y={}",p1.x,p1.y);
    p1.distance_from_origin();

}

//测试函数
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_struct_point_impl() {
        let p = Point{x: 5.0,y: 5.0 };
        println!("{:?}",p);

    }
}