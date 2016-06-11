EESchema Schematic File Version 2
LIBS:power
LIBS:device
LIBS:transistors
LIBS:conn
LIBS:linear
LIBS:regul
LIBS:74xx
LIBS:cmos4000
LIBS:adc-dac
LIBS:memory
LIBS:xilinx
LIBS:microcontrollers
LIBS:dsp
LIBS:microchip
LIBS:analog_switches
LIBS:motorola
LIBS:texas
LIBS:intel
LIBS:audio
LIBS:interface
LIBS:digital-audio
LIBS:philips
LIBS:display
LIBS:cypress
LIBS:siliconi
LIBS:opto
LIBS:atmel
LIBS:contrib
LIBS:valves
LIBS:nxp_armmcu
LIBS:ab2_7segment
LIBS:ab2_audio
LIBS:ab2_buffer
LIBS:ab2_capacitor
LIBS:ab2_connectivity
LIBS:ab2_dac
LIBS:ab2_diode
LIBS:ab2_gpio_expansion
LIBS:ab2_header
LIBS:ab2_idc
LIBS:ab2_inductor
LIBS:ab2_input_devices
LIBS:ab2_jumper
LIBS:ab2_lcd
LIBS:ab2_led
LIBS:ab2_memory
LIBS:ab2_opamp
LIBS:ab2_pot
LIBS:ab2_power
LIBS:ab2_regulator
LIBS:ab2_relay
LIBS:ab2_resistor
LIBS:ab2_sensor
LIBS:ab2_stepper
LIBS:ab2_supply
LIBS:ab2_terminal_block
LIBS:ab2_test
LIBS:ab2_transistor
LIBS:ab2_uC
LIBS:ab2_usb
LIBS:ab2_xtal
LIBS:ac-dc
LIBS:actel
LIBS:Altera
LIBS:analog_devices
LIBS:brooktre
LIBS:cmos_ieee
LIBS:dc-dc
LIBS:diode
LIBS:elec-unifil
LIBS:ESD_Protection
LIBS:ftdi
LIBS:gennum
LIBS:graphic
LIBS:hc11
LIBS:ir
LIBS:Lattice
LIBS:logo
LIBS:maxim
LIBS:microchip_dspic33dsc
LIBS:microchip_pic10mcu
LIBS:microchip_pic12mcu
LIBS:microchip_pic16mcu
LIBS:microchip_pic18mcu
LIBS:microchip_pic32mcu
LIBS:motor_drivers
LIBS:msp430
LIBS:nordicsemi
LIBS:onsemi
LIBS:Oscillators
LIBS:powerint
LIBS:Power_Management
LIBS:pspice
LIBS:references
LIBS:relays
LIBS:rfcom
LIBS:sensors
LIBS:silabs
LIBS:SparkFun
LIBS:stm8
LIBS:stm32
LIBS:supertex
LIBS:switches
LIBS:transf
LIBS:ttl_ieee
LIBS:video
LIBS:Xicor
LIBS:Zilog
LIBS:Symbols_DCDC-ACDC-Converter_RevC_20Jul2012
LIBS:Symbols_EN60617_13Mar2013
LIBS:Symbols_EN60617-10_HF-Radio_DRAFT_12Sep2013
LIBS:Symbols_ICs-Diskrete_RevD10
LIBS:Symbols_ICs-Opto_RevB_16Sep2013
LIBS:Symbols_Microcontroller_Philips-NXP_RevA_06Oct2013
LIBS:SymbolsSimilarEN60617+oldDIN617-RevE8
LIBS:Symbols_Socket-DIN41612_RevA
LIBS:Symbols_Transformer-Diskrete_RevA
LIBS:Arduino_As_Uno-cache
LIBS:boosterpack
LIBS:boosterpack40-cache
LIBS:spinny1-cache
EELAYER 25 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L LPC1114FN28/102 U?
U 1 1 569F4957
P 4850 3800
F 0 "U?" V 5250 3050 60  0000 C CNN
F 1 "LPC1114FN28/102" V 5100 3150 60  0000 C CNN
F 2 "" H 4850 3800 60  0000 C CNN
F 3 "" H 4850 3800 60  0000 C CNN
	1    4850 3800
	1    0    0    -1  
$EndComp
$Comp
L AB2_TLC5940_PWP TLC?
U 1 1 569F4CF0
P 5200 6450
F 0 "TLC?" H 5200 5600 60  0000 C CNN
F 1 "AB2_TLC5940_PWP" H 5200 7300 60  0000 C CNN
F 2 "" H 5200 6450 60  0000 C CNN
F 3 "" H 5200 6450 60  0000 C CNN
	1    5200 6450
	1    0    0    -1  
$EndComp
$Comp
L LED D?
U 1 1 569F4D6D
P 6100 5600
F 0 "D?" H 6100 5700 50  0000 C CNN
F 1 "LED" H 6100 5500 50  0000 C CNN
F 2 "" H 6100 5600 50  0000 C CNN
F 3 "" H 6100 5600 50  0000 C CNN
	1    6100 5600
	1    0    0    -1  
$EndComp
$Comp
L LED D?
U 1 1 569F4E6C
P 6100 5900
F 0 "D?" H 6100 6000 50  0000 C CNN
F 1 "LED" H 6100 5800 50  0000 C CNN
F 2 "" H 6100 5900 50  0000 C CNN
F 3 "" H 6100 5900 50  0000 C CNN
	1    6100 5900
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR?
U 1 1 569F4EB2
P 6900 5900
F 0 "#PWR?" H 6900 5650 50  0001 C CNN
F 1 "GND" H 6900 5750 50  0000 C CNN
F 2 "" H 6900 5900 50  0000 C CNN
F 3 "" H 6900 5900 50  0000 C CNN
	1    6900 5900
	1    0    0    -1  
$EndComp
Wire Wire Line
	6900 5900 6300 5900
Wire Wire Line
	6300 5600 6900 5600
Wire Wire Line
	6900 5600 6900 5900
Wire Wire Line
	5900 5900 5700 5900
Wire Wire Line
	5700 5800 5900 5800
Wire Wire Line
	5900 5800 5900 5600
Wire Wire Line
	5650 4400 9050 4400
Wire Wire Line
	5650 4500 6450 4500
Wire Wire Line
	6450 4500 6450 3400
Wire Wire Line
	2800 4400 4650 4400
Wire Wire Line
	2800 3400 7400 3400
Wire Wire Line
	4650 4500 3600 4500
$Comp
L GND #PWR?
U 1 1 569F4FD5
P 7300 4400
F 0 "#PWR?" H 7300 4150 50  0001 C CNN
F 1 "GND" H 7300 4250 50  0000 C CNN
F 2 "" H 7300 4400 50  0000 C CNN
F 3 "" H 7300 4400 50  0000 C CNN
	1    7300 4400
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR?
U 1 1 569F4FF4
P 3600 4500
F 0 "#PWR?" H 3600 4250 50  0001 C CNN
F 1 "GND" H 3600 4350 50  0000 C CNN
F 2 "" H 3600 4500 50  0000 C CNN
F 3 "" H 3600 4500 50  0000 C CNN
	1    3600 4500
	1    0    0    -1  
$EndComp
$Comp
L LD33V U?
U 1 1 569F5093
P 7400 3050
F 0 "U?" H 7450 3100 60  0000 C CNN
F 1 "LD33V" H 7400 3050 60  0000 C CNN
F 2 "" H 7400 3050 60  0000 C CNN
F 3 "" H 7400 3050 60  0000 C CNN
	1    7400 3050
	1    0    0    -1  
$EndComp
Wire Wire Line
	7300 4400 7300 3350
Connection ~ 7300 4400
Wire Wire Line
	7400 3400 7400 3350
Connection ~ 6450 3400
$Comp
L ARDUINO_SERIAL_PROGRAMLOCK JP?
U 1 1 569F57AF
P 8750 3150
F 0 "JP?" H 8550 3580 50  0000 L BNN
F 1 "Serial" H 8455 2750 50  0000 L BNN
F 2 "SparkFun-1X06_LOCK" H 8700 2700 50  0001 C CNN
F 3 "" H 8750 3150 60  0000 C CNN
	1    8750 3150
	0    1    1    0   
$EndComp
Wire Wire Line
	9050 4400 9050 3350
Wire Wire Line
	8850 3350 8850 3550
Wire Wire Line
	7500 3350 7500 3550
Wire Wire Line
	7500 3550 8850 3550
Wire Wire Line
	5650 5000 8650 5000
Wire Wire Line
	8650 5000 8650 3350
Wire Wire Line
	8750 5100 8750 3350
Wire Wire Line
	5450 5100 8750 5100
Wire Wire Line
	4700 5700 2800 5700
Wire Wire Line
	2800 5700 2800 3400
Connection ~ 2800 4400
Wire Wire Line
	4650 3900 3400 3900
Wire Wire Line
	3400 3900 3400 6150
Wire Wire Line
	3400 6150 4700 6150
Wire Wire Line
	4650 3800 3300 3800
Wire Wire Line
	3300 3800 3300 6500
Wire Wire Line
	3300 6500 4700 6500
Wire Wire Line
	5650 4200 6350 4200
Wire Wire Line
	6350 4200 6350 3200
$Comp
L SW_PUSH Reset
U 1 1 569F71ED
P 6050 3200
F 0 "Reset" H 6200 3310 50  0000 C CNN
F 1 " " H 6050 3120 50  0000 C CNN
F 2 "" H 6050 3200 50  0000 C CNN
F 3 "" H 6050 3200 50  0000 C CNN
	1    6050 3200
	1    0    0    -1  
$EndComp
$Comp
L R R
U 1 1 569F7570
P 5600 3200
F 0 "R" V 5680 3200 50  0000 C CNN
F 1 "220" V 5600 3200 50  0000 C CNN
F 2 "" V 5530 3200 50  0000 C CNN
F 3 "" H 5600 3200 50  0000 C CNN
	1    5600 3200
	0    1    1    0   
$EndComp
Wire Wire Line
	5450 3200 5200 3200
$Comp
L GND #PWR?
U 1 1 569F77AE
P 5200 3200
F 0 "#PWR?" H 5200 2950 50  0001 C CNN
F 1 "GND" H 5200 3050 50  0000 C CNN
F 2 "" H 5200 3200 50  0000 C CNN
F 3 "" H 5200 3200 50  0000 C CNN
	1    5200 3200
	1    0    0    -1  
$EndComp
$EndSCHEMATC
