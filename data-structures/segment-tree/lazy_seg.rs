
#![allow(dead_code,unused_variables)]
use std::io;

#[inline(always)]
fn cin<T>()->T
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug     //ouput of Err type of FromStr is passed to Debug to unwrap()
    {
        let mut val=String::new();
        io::stdin().read_line(&mut val).unwrap();
        let val:T=val.trim().parse().unwrap();
        return val;
    }


//read entire line 
#[inline(always)]
fn vin<T>()->Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug     //ouput of Err type of FromStr is passed to Debug to unwrap()
    {
        let mut val=String::new();
        let mut v:Vec<T>=Vec::new();
        io::stdin().read_line(&mut val).unwrap();
        for words in val.split_whitespace(){
            v.push(words.trim().parse::<T>().unwrap());
        }
        return v;
    }
//////////////////////////////////////////////////////////


fn build(l:usize,pos:usize,r:usize,v:&Vec<i64>,seg:&mut Vec<i64>){
    if l==r{seg[pos]=v[l];return;}
    let midi:usize=l+((r-l)>>1);
    build(l,pos<<1,midi,v,seg);
    build(midi+1,pos<<1|1,r,v,seg);
    seg[pos]=seg[pos<<1]+seg[pos<<1|1];
}

// fn update(l:usize,pos:usize,r:usize,seg:&mut Vec<i64>,qpos:usize,val:i64){
//     if l==r {seg[pos]=val;return;}
//     let midi:usize=l+((r-l)>>1);
//     if qpos>midi    {update(midi+1,pos<<1|1,r,seg,qpos,val);}
//     else            {update(l,pos<<1,midi,seg,qpos,val);}
//     seg[pos]=seg[pos<<1]+seg[pos<<1|1];
// }

fn push_down(l:usize,pos:usize,r:usize,seg:&mut Vec<i64>,lazy:&mut Vec<i64>){
    if lazy[pos]!=0{
        seg[pos]+=lazy[pos]*(r-l+1) as i64;
        if l!=r{
            lazy[pos<<1]+=lazy[pos];
            lazy[pos<<1|1]+=lazy[pos];
        }
        lazy[pos]=0;
    }
}

fn lazy_update(l:usize,pos:usize,r:usize,seg:&mut Vec<i64>,lazy:&mut Vec<i64>,ql:usize,qr:usize,val:i64){
    push_down(l,pos,r,seg,lazy);
    if l>qr || r<ql {return;}
    if l>=ql && r<=qr{
        lazy[pos]=val;push_down(l,pos,r,seg,lazy);
        return;
    }
    let midi:usize=l+((r-l)>>1);
    lazy_update(l,pos<<1,midi,seg,lazy,ql,qr,val);
    lazy_update(midi+1,pos<<1|1,r,seg,lazy,ql,qr,val);
    seg[pos]=seg[pos<<1]+seg[pos<<1|1];
}



fn query(l:usize,pos:usize,r:usize,seg:&mut Vec<i64>,lazy:&mut Vec<i64>,ql:usize,qr:usize)->i64{
   push_down(l,pos,r,seg,lazy);
   if l>qr || r<ql      {return 0i64;}
   if l>=ql && r<=qr    {return seg[pos];}
   let midi:usize=l+((r-l)>>1);
   return query(l,pos<<1,midi,seg,lazy,ql,qr)+query(midi+1,pos<<1|1,r,seg,lazy,ql,qr);
}


fn main(){

    let n:usize=cin();
    let q:i32=cin();

    let v:Vec<i64>=vin();
    let mut seg:Vec<i64>=vec![0;4*n];
    let mut lazy:Vec<i64>=vec![0;4*n];
    build(0usize,1usize,n-1,&v,&mut seg);

    for _queries in 0..q{
        let ch:i8=cin();
        if ch%2==1{
            let mut ql:usize=cin();
            let mut qr:usize=cin();
            ql-=1;qr-=1;
            println!("{}",query(0usize,1usize,n-1,&mut seg,&mut lazy,ql,qr));
        }
        else{
            let mut ql:usize=cin();
            let mut qr:usize=cin();
            ql-=1;qr-=1;
            let val:i64=cin();
            lazy_update(0usize,1usize,n-1,&mut seg,&mut lazy,ql,qr,val);
        }
    }


}
