# OneST

For verification of UEN according to <https://www.uen.gov.sg/ueninternet/faces/pages/admin/aboutUEN.jspx>

Upon building, navigate to target and run the executable based off your computer's specific hardware.

## Overview of files

main.rs contains a simple ui for the user to input UEN number, and to display results.
Upon pressing the enter key, main.rs calls for the verify function in verify.rs, which does the verification checks

verify takes in a UEN number in the form of a string, and returns a message detailing potential problems (String), and whether verification was valid (bool).

Dependencies used are stated in Cargo.toml
