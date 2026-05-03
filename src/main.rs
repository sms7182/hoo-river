use std::io::{stdout, Write,Stdout};
use std::{time::Duration,thread};
use rand::Rng;
use rand::thread_rng;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, KeyCode};
use crossterm::terminal::{ Clear, disable_raw_mode, enable_raw_mode};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition},
    terminal::{size},QueueableCommand,
    event::{poll,read,Event},
};

struct  Enemy{
  l:u16,
  c:u16
}

struct  Fuel{
    l:u16,
    c:u16
}
struct Bullet{
    l:u16,
    c:u16,
    energy:u16
}
struct World{
    maxc:u16,
    maxl:u16,
    player_c:u16,
    player_l:u16,
    map:Vec<(u16,u16)>,
    died:bool,
    next_start:u16,
    next_end:u16,
    enemy:Vec<Enemy>,
    bullet:Vec<Bullet>,
    gas:u16,
    score:u16,
    fuel:Vec<Fuel>
}

fn draw(mut sc: &Stdout,mut world: &World)->std::io::Result<()>{
   sc.queue(Clear(crossterm::terminal::ClearType::All))?;
 
   for l in 0..world.map.len(){
         sc.queue(MoveTo(0,l as u16))?;
         sc.queue(Print("*".repeat(world.map[l].0 as usize)))?;

         sc.queue(MoveTo(world.map[l].1,l as u16))?;
         sc.queue(Print("*".repeat((world.maxc-world.map[l].1) as usize)))?;
  
   }
    sc.queue(MoveTo(2,2))?;
    sc.queue(Print(format!("Score:{}",world.score)))?;
    sc.queue(MoveTo(2,3))?;
    sc.queue(Print(format!("Gas:{}",world.gas)))?;

   for e in &world.enemy{
        sc.queue(MoveTo(e.c,e.l as u16))?;
         sc.queue(Print("E"))?;
  
   }
   for f in &world.fuel   {
       sc.queue(MoveTo(f.c,f.l as u16))?;
       sc.queue(Print("F"))?;
   }

   for b in &world.bullet{
     sc.queue(MoveTo(b.c,b.l as u16))?
     .queue(Print('|'))?
     .queue(MoveTo(b.c,b.l-1 as u16))?
     .queue(Print('^'))?;

   }
    sc.queue(MoveTo(world.player_c,world.player_l))?;
    sc.queue(Print("M"))?;
  
   sc.flush()?;
   Ok(())

}

fn physics(mut world:World)->std::io::Result<World>{
    if world.gas<=0{
        world.died=true;
    }
    if world.player_c <= world.map[world.player_l as usize].0||
        world.player_c>=world.map[world.player_l as usize].1{
        world.died=true;
    }
    for i in (0..world.fuel.len()).rev(){
        if world.fuel[i].l==world.player_l&&world.fuel[i].c==world.player_c{
            world.gas+=10;
        }
        for j in (0..world.bullet.len()).rev(){
            if (world.bullet[j].l==world.fuel[i].l ||world.bullet[j].l-1==world.fuel[i].l)&&world.bullet[j].c==world.fuel[i].c{
                world.fuel.remove(i);
                world.score+=10;
            }
        }
    }

    for i in (0..world.enemy.len()).rev(){
        if world.enemy[i].l==world.player_l&&world.enemy[i].c==world.player_c{
            world.died=true;
        }
        for j in (0..world.bullet.len()).rev(){
            if (world.bullet[j].l==world.enemy[i].l ||world.bullet[j].l-1==world.enemy[i].l)&&world.bullet[j].c==world.enemy[i].c{
                world.enemy.remove(i);
                world.score+=10;
            }
        }
    }

    for i in (0..world.bullet.len()).rev(){
        if world.bullet[i].energy==0 ||world.bullet[i].l<3{
            world.bullet.remove(i);
        }else{
            world.bullet[i].energy-=1;
            world.bullet[i].l-=2;
            if world.bullet[i].l<2{
                world.bullet[i].energy=0;
            }
        }
    }

    for l in (0..world.map.len()-1).rev(){
        world.map[l+1]=world.map[l];
    }
    if world.next_end<world.map[0].1{
        world.map[0].1-=1;
     
    }
    if world.next_end>world.map[0].1{
        world.map[0].1+=1;
    
    }

    if world.next_start<world.map[0].0{
        world.map[0].0-=1;
     
    }
    if world.next_start>world.map[0].0{
        world.map[0].0+=1;
    
    }
        let mut rng= rand::thread_rng();
  
  if rng.gen_range(0..10)>7{
    if world.next_start==world.map[0].0&&world.next_end==world.map[0].1{
        if world.map[0].0>5 &&world.map[0].1>5{        world.next_start=rng.gen_range(world.map[0].0-5..world.map[0].1-5);}
        if world.map[0].0+5<world.map[0].1+5{
        world.next_end=rng.gen_range(world.map[0].0+5..world.map[0].1+5);}
         if world.next_end.abs_diff(world.next_start)<3{
            world.next_end+=3;
         }
    }
    
}   
for i in (0..world.enemy.len()).rev(){
    if world.enemy[i].l<world.maxl{
        world.enemy[i].l+=1;
    }else{
        world.enemy.remove(i);
    }
}

for i in (0..world.fuel.len()).rev(){

    if world.fuel[i].l<world.maxl{
        world.fuel[i].l+=1;
    }else{
        world.fuel.remove(i);
    }
}

    if rng.gen_range(0..10)>=9{
        let new_c=rng.gen_range(world.map[0].0..world.map[1].1);

        world.enemy.push(Enemy{
            c:new_c,
            l:0
        })
    }

    if rng.gen_range(0..10)>7{
        let new_c_f=rng.gen_range(world.map[0].0..world.map[1].1);
        if new_c_f>0{
        world.fuel.push(Fuel{
            c:new_c_f,
            l:0
        })
    }
    }
    if world.gas>=1{
    world.gas-=1;

    }
 Ok(world)
}


fn main() -> std::io::Result<()>{
    let mut sc:Stdout = stdout();
    let (maxc,maxl)=size().unwrap();
   
    sc.execute(Hide)?;
   
    enable_raw_mode();
    //init the screen
   


    //init the game
    let mut world=World{
        maxc:maxc,
        maxl:maxl,
        player_c:maxc/2,
        player_l:maxl-1,
        map:vec![((maxc/2)-5,(maxc/2)+5);maxl as usize],
        died:false,
        next_start:maxc/2-25,
        next_end:maxc/2+25,
        enemy:vec![],
        bullet:vec![],
        gas:100,
        score:0,
        fuel:vec![]


    };

    while !world.died{
        if poll(Duration::from_millis(10))?{
            let key=read().unwrap();
            while poll(Duration::from_millis(0)).unwrap() {
                let _=read();
            }
            match key{
                Event::Key(event)=>{
                    match event.code {
                        KeyCode::Char('q')=>{
                            break;
                        },
                        KeyCode::Left=>{
                              if world.player_c>1 {world.player_c-=1;}
                        },
                        KeyCode::Right=>{
                              if world.player_c<maxc {world.player_c+=1;}
                        },
                        KeyCode::Up=>{
                            if world.player_l>1 {world.player_l-=1;}
                        },
                        KeyCode::Down=>{
                              if world.player_l<maxl {world.player_l+=1;}
                        },
                        KeyCode::Char(' ')=>{
                            if world.bullet.len()==0{
                                world.bullet.push(Bullet{
                                    l:world.player_l+1,
                                    c:world.player_c,
                                    energy:maxl/2
                                })
                            }
                        }
                        _=>{

                        }
                        
                    }
                },
                _=>{},
            }
        }else{

        }

       world= physics(world).unwrap();


        draw(&sc,&world)?;
        thread::sleep(Duration::from_millis(100));

    }


    sc.execute(Show)?;

    disable_raw_mode()?;

   sc.execute(Clear(crossterm::terminal::ClearType::All))?;
   sc.execute(Print("Thanks moji for playing!!!!!!!!!!!!!!!"))?;
   

    Ok(())
}
