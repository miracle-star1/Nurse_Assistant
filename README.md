# Medication Reminder Tool

This Rust-based command-line application helps nurses keep track of medication schedules for patients. It provides a simple timer that reminds the user when it's time to administer medication.

## Features
- **Patient Name Input**: Keeps track of the patient who needs the medication.
- **Time Configuration**: Allows setting a specific hour and minute for the reminder.
- **Automatic Notification**: Prints a message when it’s time to administer the medication.
- **CPU-Friendly**: Uses a 30-second sleep interval to minimize resource usage.

## How It Works
1. The user enters the patient’s name and the medication time (hour and minute).
2. The program continuously checks the current time.
3. When the set time matches the current time, a reminder is displayed.

## Prerequisites
- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).

## Installation
1. Clone this repository or copy the `medication_reminder.rs` file.
2. Navigate to the project directory.
3. Run the following command to build the project:
   ```bash
   cargo build
   ```

## Usage
1. Run the program using:
   ```bash
   cargo run
   ```
2. Follow the prompts to:
   - Enter the patient’s name.
   - Set the medication time (hour and minute in 24-hour format).
3. Wait for the reminder message to appear at the specified time.

## Example
```bash
Welcome to the Medication Reminder Tool!
Enter the patient's name:
Mercy Miracle
Enter the hour (24-hour format) for the medication:
14
Enter the minute for the medication:
30
Reminder set for Mercy Miracle at 14:30.
Time to give medication to John Doe!
```

## Limitations
- The program must remain running for the reminder to work.
- No recurring reminders; the program only handles a single reminder at a time.

## Future Enhancements
- Add support for multiple reminders.
- Integrate with a graphical user interface (GUI).
- Enable recurring medication schedules.

## Contributing
Feel free to fork the repository and submit pull requests for improvements.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.
