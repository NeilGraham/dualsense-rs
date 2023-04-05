# HID Reference

HID Stands for Human Interface Device. It is a protocol that allows for communication between a computer and a device. The Dualsense controller uses HID to communicate with the computer.

We will be using `data` to represent the `0u8; 64` (length 64 array of unsigned bytes with default values of 0) that is returned by the Dualsense `device.read_timeout` method. This array represents all information being sent by the Dualsense controller.

```
 0: 49    1: 17    2: 129   3: 127   4: 127   5: 130  
 6: 0     7: 0     8: 1     9: 8    10: 0    11: 0    
12: 0    13: 90   14: 86   15: 174  16: 78   17: 250  
18: 255  19: 0    20: 0    21: 0    22: 0    23: 81   
24: 1    25: 56   26: 31   27: 241  28: 4    29: 113  
30: 185  31: 196  32: 48   33: 17   34: 191  35: 21   
36: 5    37: 0    38: 192  39: 40   40: 0    41: 2    
42: 208  43: 9    44: 9    45: 0    46: 0    47: 0    
48: 0    49: 0    50: 19   51: 190  52: 196  53: 48   
54: 7    55: 0    56: 0    57: 120  58: 91   59: 80   
60: 35   61: 217  62: 163  63: 227
```
- Instance of the `data` array.

---

# Analog Stick Positions (`data[2]` - `data[5]`)

`data[2]` represents the left analog stick's X-axis position.

`data[3]` represents the left analog stick's Y-axis position.

`data[4]` represents the right analog stick's X-axis position.

`data[5]` represents the right analog stick's Y-axis position.

X-axis positions range from 0 (left) to 255 (right), with 127 being the center position.

Y-axis positions range from 0 (up) to 255 (down), with 127 being the center position.

---

## Trigger Actuation (`data[6]` - `data[7]`)

`data[6]` is the byte that contains every variation of left trigger (L2) actuation.

`data[7]` is the byte that contains every variation of right trigger (R2) actuation.

Both bytes return 0 when the trigger is not presssed, and 255 when the trigger is fully pressed.

--- 

## Front-Facing Buttons (`data[9]`)

`data[9]` is the byte that contains every variation of front-facing button presses.

`data[9] = 8` when no buttons are pressed.

All shape buttons can be pressed at the same time.

Only one of the D-Pad options below can be true at a time.

| Button | Modifier | If Only Pressed |
|--------|----------| ----------------|
| D-Pad Up | -8 | 0 |
| D-Pad Up-Right | -7 | 1 |
| D-Pad Right | -6 | 2 |
| D-Pad Down-Right | -5 | 3 |
| D-Pad Down | -4 | 4 |
| D-Pad Down-Left | -3 | 5 |
| D-Pad Left | -2 | 6 |
| D-Pad Up-Left | -1 | 7 |
| No Button Pressed | 0 | 8 |
| Square | +16 | 24 |
| Cross (X) | +32 | 40 |
| Circle (O) | +64 | 72 |
| Triangle | +128 | 136 |

---

## Shoulder Buttons + Analog Stick Presses + Start/Share Buttons (`data[10]`)

`data[10]` is the byte that contains every variation of shoulder button presses

All buttons can be pressed at the same time.

| Button | Modifier |
|--------|----------|
| L1 | +1 |
| R1 | +2 |
| L2 | +4 |
| R2 | +8 |
| Share | +16 |
| Start | +32 |
| L3 | +64 |
| R3 | +128 |

---

## Home Button (`data[11]`)

`data[11]` is the byte that contains every variation of home button presses.

`data[11] = 0` when no buttons are pressed.

`data[11] = 1` when the home button is pressed.