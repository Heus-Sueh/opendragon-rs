# Hardware Validation Ladder and Investigation Journal

## The Hardware Validation Ladder

Report the highest level of validation actually reached:

1. **Static Spec / Heuristic**: Hypothesized based on another device or vendor string.
2. **Compilation**: Code compiles and types check. (Not proof that hardware accepts it!)
3. **Descriptor Decoded**: HID Report Descriptor parsed and confirmed report ID and field sizes.
4. **Packet Unit Test**: Unit test verifies byte layout against a known capture fixture.
5. **Read-Only Probe**: Executed non-destructive probe (`opendragon probe`) against real hardware.
6. **Command Sent Without Error**: HID transport transmitted packet without OS/hardware stall.
7. **Negative Control Verified**: Varied parameter (e.g. green vs blue) and confirmed distinct behavior.
8. **Physical Hardware Verified**: Observable physical response verified on the target device (LED illuminated, switch depth altered, or profile switched).
9. **Multi-Session Reproducibility**: Another investigator or test run reproduced the behavior reliably.

A lower level is never evidence for a higher one. A successful build does not prove that the keyboard changes color, and an accepted write does not prove the MCU applied the setting.

## Negative Controls

Whenever practical, prove that an interpretation can be falsified:
- If bytes `[4..6]` are believed to be `[R, G, B]`, test sending pure red (`#FF0000`) and verify green/blue are off. Then send pure green (`#00FF00`) and verify red is off.
- If byte `[2]` is believed to be mode index, verify that sending invalid or out-of-bounds mode numbers is rejected or does not match.

## Investigation Note Format

For every significant hardware investigation, create a Markdown document under `re/devices/<device>/investigations/NNNN-<title>.md`:

```markdown
# Investigation NNNN: <Title>

- **Target**: <Device Name> (<VID:PID:bcdDevice>)
- **Date**: YYYY-MM-DD
- **Author**: <Name or Agent ID>
- **Hardware Environment**: OS, kernel, USB topology
- **Software Used**: Official tool version, capture method

## 1. Observable Target
What physical behavior is being investigated?

## 2. Hypothesis
What is the falsifiable prediction about the protocol, packet, or offset?

## 3. Raw Observations & Captures
Include exact packet hex strings, offsets, and report IDs.

## 4. Controlled Diff & Analysis
Comparison between baseline and modified state.

## 5. Implementation & Validation Level
Highest validation level reached on the ladder.

## 6. Remaining Uncertainty & Next Target
What is still unconfirmed, and what experiment should be run next?
```
