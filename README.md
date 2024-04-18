# Image Ascii
Image to Ascii Converter - this project was mainly to gain some familiarity with godot

## Table of Contents
- [Examples](#examples)
- [Releases](https://github.com/FrankWan27/ImageAscii/releases/)
- [Build](#build)

## Examples
![Bunny](https://github.com/FrankWan27/ImageAscii/blob/main/img/bunny.png?raw=true)
```
                                                                                                              
                                               ______                                                         
                         _____             _,qMMMMMMMMp_                                                      
                      _qpMMMMMMp_        _qMMMMMMMMMMMMMp                                                     
                    qMMMMMMMMMMMMp_     _MMMMM       "MMM#                                                    
                   qMMM#"     "MMMM_   qMMM#          qMM#                                                    
                  qMMMK         #MMB  qMMM#           qMM#                                                    
                  MMMW           MMM# MMM#            qMMMp_                                                  
                 qMMM|           JMMMMMMM             #MMMMMMpppw,,___                                        
                 [MMM|            #MMMMMK             MMMMMMMMMMMMMMMMMMpw__                                  
                 (MMM|            JMMMMM             qMMMW       """MMMMMMMMMp__                              
                 [MMM|             WMMM#             #MMW               "MMMMMMMMp_    __aqMMMMMp,__          
                  MMM|             JMMMh             MMMp                    "MMMMMp,qMMMMMMMMMMMMMMp_        
                  MMMp              MMM             qMMMp                       "MMMMMMMMK      "MMMMMp_      
                  MMMB              MM#             #MMM                           "MMMMMW_        MMMMM,     
                 qMMMM              MMK             MMM"                             "MMMMMp_       "MMMM,    
                _MMMMMp              "              M""                                "MMMMMp       7MMM#    
              _qMMMMMM#                                                                  "MMMMp_      MMM#    
             qMMMM" MMM                                                                    "MMMB_    qMMM#    
           _#MMMM    "#t                                                                    ^MMMM___qMMMW     
          _MMMM"                                                                              MMMMMMMMMM      
         qMMMW                                                                                 MMMMMM"        
        yMMMW                                                                                   MMMMp         
       yMMM#                                                                                     #MMM         
      _MMMM                                                                                       MMMp        
      #MMM                                                                                        #MMM_       
     qMMMK                                                                                         MMM#       
     MMMM                                                                                          3MMM       
    qMMM#                                                                                           MMMp      
    #MMMt                                                                                           #MMB      
    MMMM                                                                                            #MMM      
    MMM#                                                                                            #MMMp     
    MMM#                                                                                            #MMMp     
    MMMB                                                                                            #MMMp     
    MMMM                                                                                            MMMMp     
    MMMM                                                                                            MMMM      
    #MMMp                                                                                          qMMMW      
    JMMMB    _qpp,_                               __ag,_                                           MMMM#      
     #MMMp   MMMMMM                              qMMMMMMp                                         qMMMM       
      MMMM,  #MMMMMp                            qMMMMMMMK                                        _MMMM"       
      "MMMMp  """"""            ,qp              MMMMMM"                                        _MMMMK        
       "MMMMp                __qMMMMp_                                                         yMMMMK         
         #MMMB_             qMMMMMMMMMB                                                      _MMMMM"          
          "MMMMB_            """"  "MM^                                                    _qMMMMM            
            "MMMMMp,_                                                                   _qMMMMMM              
              "MMMMMMMw__                                                           __qMMMMMM"                
                 "MMMMMMMMMpw____                                             __,qpMMMMMMMM                   
                     "MMMMMMMMMMMMMpppg,________               ______,agqpMMMMMMMMMMMMM"                      
                           "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM""                            
                                    """MMMMMMMMMMMMMMMMMMMMMMMMMMMMMM""""
```
![NkoPog](https://github.com/FrankWan27/ImageAscii/blob/main/img/nkoPog.png?raw=true)
For white text on a dark background, you can generate the Ascii with invert toggled on:
![NkoPogInvert](https://github.com/FrankWan27/ImageAscii/blob/main/img/nkoPogInvert.png?raw=true)
```
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMW"          WMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMW"        WMMMMMMMMW"    ______    WMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMM"     ___     WMMMMM"    _pMMMMMMg    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMM    _qMMMMMp    #MMM"   _qMMMMMMMMM)   MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMM'   qMMMMMMMMB_   WM"   _MMMMMMMMMMM)   MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMM#    BMMMMMMMMMB    W    BMMMMMMMMMMMb   "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMp   qMMMMMMMMMMMp       #MMMMMMMMMMMW        """""WWWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMp   qMMMMMMMMMMMM      qMMMMMMMMMMMM#     ___          ""WMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMb   qMMMMMMMMMMMMp     #MMMMMMMMMMMM)   _MMMMMMMpp,__       ""MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMp   qMMMMMMMMMMMMM    qMMMMMMMMMMMMM    BMMMMMMMMMMMMMMp,_      "WMMMMW"""     ""WMMMMMMMMMMM
MMMMMMMMMMMMMMMMM#   qMMMMMMMMMMMMM)   #MMMMMMMMMMMM#   (MMMMMMMMMMMMMMMMMMMp,_     ""     _         MMMMMMMMM
MMMMMMMMMMMMMMMMMM   (MMMMMMMMMMMMM#   #MMMMMMMMMMMM)   (MMMMMMMMMMMMMMMMMMMMMMMp_       _MMMMMMp_     WMMMMMM
MMMMMMMMMMMMMMMMMW    WMMMMMMMMMMMM#   MMMMMMMMMMMMM    #MMMMMMMMMMMMMMMMMMMMMMMMMpp_     WMMMMMMMq_    "MMMMM
MMMMMMMMMMMMMMMMM#    #MMMMMMMMMMMMM  qMMMMMMMMMMMMW   qMMMMMMMMMMMMMMMMMMMMMMMMMMMMMp_    "MMMMMMMMp    #MMMM
MMMMMMMMMMMMMMMM"     (MMMMMMMMMMMMMpqMMMMMMMMMMMMMM,,qMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMp_    "MMMMMMM)    MMMM
MMMMMMMMMMMMMMW        WMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMp_    WMMMMMN    MMMM
MMMMMMMMMMMMM"    qA   #MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMp    WMMMM"   qMMMM
MMMMMMMMMMMW    _MMMp,_qMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMq    "MW"   _MMMMM
MMMMMMMMMM"    pMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMB_        _MMMMMM
MMMMMMMMM"    #MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM,    _qMMMMMMMM
MMMMMMMM"    BMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM,   #MMMMMMMMM
MMMMMMM"   _qMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM    WMMMMMMMM
MMMMMM#    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMp    MMMMMMMM
MMMMMW    #MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM,   #MMMMMMM
MMMMM"   qMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM#    MMMMMMM
MMMM#    #MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMp   #MMMMMM
MMMM)    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM#    MMMMMM
MMMM    (MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM    WMMMMM
MMM#    #MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM    #MMMMM
MMM#    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM    (MMMMM
MMM#    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM    (MMMMM
MMM#    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM    (MMMMM
MMM#    NMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMW    (MMMMM
MMM#    #MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM#    #MMMMM
MMMM    (MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMb    NMMMMM
MMMMp    WMMMF""""MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMW"""WMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMW    qMMMMMM
MMMMM,   'MM#      WMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"      (MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM     #MMMMMM
MMMMM#    "MM_      MMMMMMMMMMMMMMMMMMMMMMMMMMMM)       _MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM#    qMMMMMMM
MMMMMMg    "MMpq,,,qMMMMMMMMMMMM"  WMMMMMMMMMMMMA_   __qMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMR    qMMMMMMMM
MMMMMMMp    "MMMMMMMMMMMMMMMMMW     "WMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"    _MMMMMMMMM
MMMMMMMMA_    NMMMMMMMMMMMMMp          MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMF     qMMMMMMMMMM
MMMMMMMMMMp     MMMMMMMMMMMMBp,,qqB,__pMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMW     _qMMMMMMMMMMM
MMMMMMMMMMMMp_     "WMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMW"     _qMMMMMMMMMMMMM
MMMMMMMMMMMMMMq_      "WMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMW"     __pMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMp__       """WMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMW""       __qMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMp___          """""WWMMMMMMMMMMMMMMMMMMMMMMMMWW"""""           ___qMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMpp,____                                            ___,qpMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMppq,,_____________         ______,,ppqqpMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
```
## Build
[Download the release and run (Windows only)](https://github.com/FrankWan27/ImageAscii/releases/)

This project uses godot and godot-rust

1. ```git clone https://github.com/FrankWan27/ImageAscii.git```
2. Build rust/ folder using `cargo build`
3. Open godot/ project in Godot and run using the editor.
