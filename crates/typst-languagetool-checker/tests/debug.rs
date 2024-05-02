use tokio::runtime::Runtime;
use typst_languagetool_checker::check_file;

const FILE: &str = r#"// Need to put in my mark, make sure that it conveys that I was the one who did it, and I encountered x or y isues
// Include code
// Demonstrate understanding

#set heading(numbering: "1.1")
#set par(justify: true)

#align(center, text(20pt)[
  *EEEN301 Lab Report*
])

#grid(columns: (1fr, 1fr), column-gutter: 20pt, align(right)[
  Trent Shailer
], align(left)[
  300602354
])

#line(length: 100%)

= Introduction

// objective
// info about board, cpu, debugger, etc
// Advanced RISC Machines (ARM)
// Texas Instruments (TI) Sitara AM3358 ARM Cortex-A8 32-bit microprocessor
// beaglebone black development board
// Joint Test Action Group (debugger) - Segger J-Link USB to JTAG debugger

= Lab 1

== Methodology

The Beaglebone was powered and set to bare-metal mode where it would just run
the program loaded to it without an operating system. A Segger J-Link JTAG
debugger was connected between the Beaglebone and the PC used throughout the
lab. Additionally, a RS323/USB cable to connect the UART on the Beaglebone to a
USB port on the PC. Code Composer Studio (CCS) was launched and used as the
development and debugging program throughout the lab.

A template 'hello world' program was first setup to display the words "Hello
World!" on the CCS terminal from the Beaglebone. This program was then modified
to print "Hello Seb!".

The next program loaded onto the Beaglebone flashed the 4 LEDs on the board on
and off in unison at a set pace. This program was then modified to change the
speed of the LEDs and then to have a new pattern.

The final program loaded onto the Beaglebone was one that echoed whatever it
received over the UART connection back. For this the PuTTY program was used to
send and receive bytes over the RS323/USB cable. This program was then modified
to display the lower 4-bits of the ASCII the Beaglebone received on the UART on
the 4 LEDs it has.

== Results

Execution of the 'hello world' program was successful, and the code was modified
as shown in @lab1_hello to print "Hello Seb!".

The flashing LEDs program was loaded, however, I ran into an issue that occurred
when attempting to debug and run the program where the code would not compile
properly. This issue turned out to be because I clicked yes when asked by CCS to
use the debug profile found in the project folder. As I was just running the
code supplied to us at this point, I deleted the project then opened a fresh
copy and selected 'no' to the prompt which fixed the problem.

Then modified this program by increasing the value of the 'TIME' variable to
make the LEDs flash slower. Then I modified it so that it would set the LEDs on
and off in a chain such that only LED one is on, it waits then turns LED one off
and two on at the same time, and so on with code as shown in @lab1_leds.

The final program was loaded successfully and without issues now that I knew to
answer 'no' to loading the debug profile. At first, I was confused that the
program was not echoing back what I entered into the PuTTY terminal as I had
expected when I pressed 'C' it would appear twice in the prompt, one from me
writing it in, and one from the Beaglebone echoing it back, however after
removing the echoing part from the code I found that what I typed into the
terminal didn't get written by default and therefore the one 'C' that appeared
was the program functioning as intended.

After first trying to isolate each bit in the lower 4 bits of the ASCII and
toggle each LED accordingly, I was prompted about the `LED_setValue` function
that sets all 4 LEDs based on a byte input. The code I ended up with as shown in
@lab1_ascii works by receiving the char (byte) from the UART port, then
performing an AND between this byte with 0b0000_1111 to mask for just the lower
4 bits, however, this step may have been unnecessary depending on how the
`LED_setValue` function works, the LEDs are then set by using the `LED_setValue`
function and passing it the masked copy of the byte received from the UART port.

#table(
  columns: 2,
  [#figure(
      image("2024-03-26-19-08-19.png", width: 100%),
      caption: [C code that prints "Hello Seb!" to the terminal.],
    ) <lab1_hello>],
  [ #figure(
      image("2024-03-26-19-20-22.png", width: 100%),
      caption: [C code that uses the GPIO ports on the Beaglebone to set its LEDs on and off in
        a chain.],
    ) <lab1_leds> ],
)
#figure(
  image("2024-03-26-19-36-37.png", width: 50%),
  caption: [C code that receives a byte from the UART port and displays the lower 4 bits on
    LEDs.],
) <lab1_ascii>

Note that code screenshots were taken after the lab on a different computer and
IDE, however, the contents of the code remain the same.


== Discussion

// Learning capabilities of the debugger
// learning how to load code/how the bb operates
// interpreting C code
// serial communications
// etc

= Conclusion
"#;

#[test]
fn debug() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let _problems = check_file(
            "https://language.trentshailer.com",
            "",
            "",
            FILE.to_string(),
            "en-GB".to_string(),
            None,
            None,
            None,
        )
        .await;
    });
}
