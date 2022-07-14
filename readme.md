# Shifting

A roguelike game I'm writing for fun.

More or less set in the same "universe" as [Burden](https://spaceshipsin.space/burden), a TTRPG I'm designing.

## Goals

### Technical

- Learn how to program in [Rust](https://www.rust-lang.org/)
- Play with some fun procedural generation algorithms

### Game design

Most of the design ideas align with the [Burden Design Guide](https://spaceshipsin.space/burden/design-guide)

- Focus on exploration
- Deemphasize/discourage combat

## Building and running the game

- Install [Rust](https://www.rust-lang.org/)
- Run `cargo xtask run` in the root folder of this repository

```
                         ,        ᴗ       ⁖
⋅                    ⋅  ѻ ⋅                     ᴗ    ⋅ᴗ       ᴗ ‿ ․      ⋅  ₍         ₍
     ,    ⚬                   ᴗ ,    ᴗ    ⋅   ₍        ∙              ᴗ        ₎  ⚬⚬●
        ,              , ᴗ        ᴗ                      ₎         ●                 ₎ ᴗ
    ᴗ        ,             ,            ⋅      ₍ ,      ⋅₍ ᴗ ●          ₍       ●        ₍
                      ᴗ ᴗ      ,      ∵     ⋅  ⋅        ₍        ᴗ      ⚬  ⋅ ,   ⋅  ,   ,
 ∵    ,                          ⋅ ⋅               o         o
   ,                                         ₎,                          ₎       ₍    ⋅
₍      ∵ ⋅   ₍       ѻ                ᴗ  ᴗ ₍         ⋅  ∙  ₍           ₎,                ,
            ₎ ∙                                    ⋅       #▓██##▓▓#█, ₍
                       ⋅                                 ∙⋅▓█▓#▓█▓██#     ₍         ₍
          ∵        ⋅                 ᴗ  ⁖                  ▓▓▓▓▓##▓▓#                    ₎
●         ₎     ₍       ●        ₎                    ⚬    ▓##█▓█#▓#▓  ₍ѻ           ₎
            ₎             , ,    ,  ᴗ            ∴         ▓█#█▓#▓#▓#    ₍             ⋅
  ∙             ⋅           ⋅         ₍          ,                                     ,
    ∙   ▓█##█#█▓▓█  ᴗ    ₎ ₍   #█▓▓#█#█▓█  ⋅⋅                             ⋅    ⋅
        ▓#█#█#█▓▓#      ₎⋅     #▓█##█▓##█             , ѻ                              ,
 ⋅      ██▓▓#█#▓▓█ ∙   ∙       ▓#█#█#▓##█                         ₍
        #████#██#▓      ₎      ▓#▓██▓▓##▓   ₎⁖           ᴗ                 ₎
      , █#▓##▓▓###             █#▓▓▓##▓▓#                #█#▓█▓▓#█▓                     ᴗ
                            ,       ₎                    ####█▓█##█⋅            ,      ₎
             ,        #█#█##▓██▓                        ₍####▓█▓#██  ᴗ⋅       ,     ᴗ
   ᴗ ,        ᴗ,   ᴗ  ▓▓███▓▓██#                       ● █▓▓█▓▓█▓█#           ⋅,
   o₎      ₎ ᴗ        ##█###█#▓▓     ₍  ⋅                #▓█##█▓█##                  ,
    ₎    #█▓###▓█▓█   █#▓██#██##                ᴗ ,                  ₎        ⋅    ₍
    ₍ ⋅ ₎#█##█▓█▓▓▓   #▓▓█▓█##█▓⚬       ⋅            ,    ₎    ⁔                 ⋅
    ₍    ▓▓▓#▓█▓##▓  ₎    ᴗ            o      ₍                    ₎       ₎
         ▓██#█#█▓#█  ᴗ        ⋅   ⋅,     ∴              ⚬     ⋅  ⋅      ₎
₎        ###▓▓###█▓                                               ₎ᴗ         ∴
                     o₍                    ⋅                   , ⋅∴  ₍ ⋅
                     ,            ⋅       ₍         ⋅   ₍    o₎ ₎
         ⋅      ,    ∵          ⋅     , ₍ ѻ                  ⚬                        o ᴗ
           ₎ ₎   ⋅                ᴗ             ⚬              #####▓#▓#█          ⋅
                          ⋅    ⋅                               ▓▓#▓█###█▓₎₍       ₍
          ₍     ₍⋅                        ₎,   o             ₍₍██▓#▓▓##▓▓
   ₎     ₍  █▓#██▓▓▓▓▓   ⋅                                     ▓▓▓█▓▓█#▓#
 ₍        , ▓██#▓▓█▓▓#   ѻo                    ․            ᴗ  ▓██##██##█            ₎
            ▓▓▓#█#▓#▓# ⁔    ₎                                               #▓█████##█ ․
            ▓#▓##█▓▓█▓  ‚  ,ѻ                              ₎              ᴗ,▓███▓██▓▓▓
 ₍      ₎   ####█#██▓▓       ∙              ,∴         ₎         #▓#██##█#█ █▓#█▓██#▓█
           ‚       ₎   ₍                  ₎   ₎ ₎      o         #▓▓#▓█##▓▓ ██▓▓█▓##▓▓
 ,   ₎ ᴗ          ,                                    ₍         #▓█▓#█##█#⋅##█▓#█▓█#█
                             ⋅                                   █#█#▓█#▓▓▓         ₍
  ⋅    ᴗ ⋅ ᴗ          ѻ ᴗ    ⋅ ⁖                         ∴ѻ      ##█#▓▓█#█▓      ₍₍
     ₍⋅   ₎                               ᴗ           C⋅₎⋅  ᴗ                   ₍       ●
```

