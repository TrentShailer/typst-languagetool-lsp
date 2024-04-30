use typst::syntax::Source;
use typst_languagetool_preprocessor::preprocess;

#[test]
fn test() {
    let source = Source::detached(
        r#"#let gap = {
  linebreak()
  linebreak()
}

#set page(header: [
  #set text(8pt, style: "italic")
  EEEN301 Assignment 1
  #h(1fr) 2024
  #h(1fr) Trent Shailer - 300602354
], footer: [
  #set align(right)
  #set text(8pt, style: "italic")
  Page
  #counter(page).display("1 of 1", both: true)
])

#set raw(tab-size: 4)
#set heading(numbering: "1.a.")
#set par(justify: true)

// 1.
=
$"cycles per second" = "clock rate (Hz)"$
// 1.a.
==
$"instructions per second (IPS)" = "cycles per second"/"cycles per instruction"$
#gap

$"IPS"_"P1" &= (2.4 times 10^9)/1.2 \
           &= 2.00 times 10^9$

$"IPS"_"P2" &= (2.2 times 10^9)/2.6 \
           &= 8.46 times 10^8$

$"IPS"_"P3" &= (1.8 times 10^9)/1.4 \
           &= 1.29 times 10^9$
#gap

P1 has the highest average instructions per second.

// 1.b.
==
$"instructions (I)" = "instructions per second" times "time"$

$"cycles (C)" = "cycles per second" times "time"$
#gap

$I_"P1" &= (2.00 times 10^9) times 10 \
       &= 2.00 times 10^10$

$C_"P1" &= (2.4 times 10^9) times 10 \
       &= 2.40 times 10^10$
#gap

$I_"P2" &= (8.46 times 10^8) times 10 \
       &= 8.46 times 10^9$

$C_"P2" &= (2.2 times 10^9) times 10 \
       &= 2.20 times 10^10$
#gap

$I_"P3" &= (1.29 times 10^9) times 10 \
       &= 1.29 times 10^10$

$C_"P3" &= (1.8 times 10^9) times 10 \
       &= 1.80 times 10^10$

// 1.c.
==
$"time" = 10 times 0.8 = 8"s"$

$"cycles per second (CR)" = "instructions"/"time" times "cycles per instruction (CPI)"$
#gap

$"CPI"_"P1" &= 1.2 times 1.25 \
           &= 1.5$

$"CR"_"P1" &= (2.00 times 10^10)/8 times 1.5 \
          &= 3.75 times 10^9 \
          &= 3.75 "GHz"$
#gap

$"CPI"_"P2" &= 2.6 times 1.25 \
           &= 3.25$

$"CR"_"P2" &= (8.46 times 10^9)/8 times 3.25\
          &= 3.44 times 10^9 \
          &= 3.44 "GHz"$
#gap

$"CPI"_"P3" &= 1.4 times 1.25 \
           &= 1.75$

$"CR"_"P3" &= (1.29 times 10^10)/8 times 1.75\
          &= 2.82 times 10^9 \
          &= 2.82 "GHz"$

// 2.
=
Subtracting from the stack pointer is for pushing new values into the stack.
Having the stack grow downwards means that all values stored in the stack have a
positive offset from the current stack pointer.// TODO explain why it is true

// 3.
=
RISC stands for Reduced Instruction Set Computer, and CISC stands for Complex
Instruction Set Computer.

CISC's methodology is based on including single instructions that execute many
low-level instructions. Whereas, RISC's methodology is to only include low-level
instructions in order to simplify the design of the system at the expense of
often requiring more code for the same operation on a CISC processor.

// 4.
=
A leaf procedure is a set of code that is called by other procedures but itself
does not call any other procedures, whereas, a non-leaf procedure is a set of
code that calls other procedures.

// 5.
=
Recursive procedures introduce more overhead because recusion has to maintain
the stack and keep functions records for each iteration.

// 6.
=
In the link register (LR)

// 7.
=
Pseudo instructions are instructions that are translated into other instructions
by the assembler. In ARM v8 the `MOV` instruction compiles down to different
instructions depending on the parameters given to it.

For example, in 32-bit arm if you are using `MOV` on some registers:

`MOV r1, r2`

Will get compiled to

`ORR r1, <Zero register>, r2`

Where `ORR` performs a bitwise OR on `r2` and stores the result in `r1`, since
the assembler puts the zero register in as part of the OR, the result is just
`r2` copied into `r1`. Whereas, when using `MOV` to or from the stack pointer:

`MOV r1, SP`

Will get compiled to

`ADD r1, SP, #0`

// 8.
=
An atomic operation is an operation on a shared resource that cannot be
interfered with by the other entities that can access the shared resource. For
example, two processors performing a non-atomic write to a shared memory address
will often result in undefined behaviour, whereas, an atomic write would ensure
that only one processor can write to the shared address at a given time.

// 9.
=
The concept of locking describes a technique to safely deal with shared data in
which a 'lock' is used alongside the shared data.

This lock is checked before an entity accesses the shared data, if the lock is
free then the entity will trigger the lock, perform its operation, then free the
lock. If the lock is locked when the entity checks it, it will continuously
check the lock until it is free.

// 10.
=
LDREX, Load Register Exclusive, is an atomic load operation if the address has
the shared memory attribute and will mark the address as exclusive access for
the processor executing the instruction ensuring a concurrent load cannot
happen.

STREX, Store Register Exclusive, is an atomic store operation where the store
will only occur if the processor that is executing the instruction has exclusive
access to the memory.

// 11.
=
One example would be to function as a lock on a larger shared memory location
between a GPU and a CPU, the CPU needs to write new vertex information to the
GPUs memory and the GPU shouldn't read that memory during write. The CPU would
use the LDREX and STREX instruction on a memory location that can be marked as
exclusive access to wait for the GPU to finish reading it if it is being read
and to then obtain its own lock on the memory. The CPU can then perform the
write to the larger memory location knowing that the GPU should check the lock
for that memory area before attempting to read from it.

This ensures that the larger memory location without exclusive access cannot be
accessed simultaneously.

// 12.
=
Not sure.

// 13.
=
If no memory addresses are marked as exclusive access for the executing
processor the STREX operation should fail, however, if a different address is
tagged as exclusive access for the executing processor it depends on the
manufacturer's implementation on whether the store is successful.

// 14.
=
By adding an S onto the keyword for the instruction, e.g. `SUBS` or `ADDS`.

// 15.
=
Arm v8 added a 64-bit architecture.

// 16.
=
The stack pointer increment on ARMv7A should be 32 bits whereas on ARMv8A should
be 64 bits.

// 17.
=
The main difference between Thumb and ARM is that Thumb instructions are mostly
16-bit compared with ARM's 32-bit instructions, this allows for more dense code
when compared to ARM.

This is useful in certain applications such as devices with limited memory as
the smaller instructions mean that the limited memory can store a larger number
of instructions so processors targeting these sorts of use cases are likely to
want to include Thumb as an option for the device manufacturers.

Along with this in situations where bus width is less than 32 bits Thumb code
may be more performant than ARM code.

// 18.
=
An integrated circuit (IC) is an electronic circuit made up of a single piece of
silicon that has been etched using photolithography.

A Microcontroller is an IC that contains a processor, memory, and I/O
peripherals.

A system on a chip is a more advanced microcontroller that integrates more
advanced peripherals like a graphics processing unit, a wifi module, and/or
other coprocessors

// 19.
=
// 19.a.
==
Joint Test Action Group

// 19.b.
==
1. Read and write the registers.
2. Read and write the memory.
3. Pause the execution of the program at certain points.
4. Step through the program line by line.

// 20.
=
One of the ideas described by Patterson and Hennessy in Computer Organisation
and Design ARM Edition is 'Performance via Prediction'. This idea puts forth
that it is often faster for the processor to predict the outcome of a condition
rather than waiting until it is certain. However, the potential performance
gains from prediction come with the caveats that for an overall performance gain
the cost of an incorrect prediction must be considered along without often the
predictions are correct.

The ARM Cortex A8 implements this idea through a technique called branch
prediction. The Cortex A8, like many processors, utilises an instruction
pipeline to improve performance. This pipeline improves performance by keeping
all parts of the processor busy by beginning to process the next instruction
before the rest of processor has finished computing the previous instructions.
This, however runs into an issue when faced with a conditional branch as there
are two possible sets of instructions that could be loaded depending on the
result of the condition.

Without branch prediction, the Cortex A8 incurs a 13-cycle penalty on every
conditional branch as the processor has to wait for the condition to be computed
before it can continue loading instructions into the pipeline. However, with
branch prediction the Cortex A8 only incurs that 13-cycle penalty on an
incorrect prediction.

Branch prediction mitigates some of this performance loss by having hardware
that will predict the outcome of a condition before it has been completely
processed. The hardware loads the instructions from the predicted branch into
the processor for computation so that if the prediction was correct there is no
downtime in the pipeline. And if the prediction was incorrect then the predicted
instructions are flushed from the pipeline and the correct instructions loaded
as if no prediction was made.
"#,
    );
    dbg!(&source.root());
    let _paragraphs = preprocess(&source);
    // dbg!(&paragraphs);
}
