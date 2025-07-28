# Cool Birthday

A very cool birthday manager written in Rust, so you never forget one ever again.

## Commands
If no command is provided, the app will check all registered birthdays.
If one matches the current date, a popup window will appear.
(This is useful if you want to run automatic checks, for example)
### Version
Show the installed version of Cool Birthday
### List
Lists all registered birthdays.
### Check
- `name` (optionnal)
  - **Without a name**:
Checks all birthdays.
If today is someone's birthday, it prints their name.
If not, it shows the next upcoming birthday.
  - **With a name**:
Prints the next birthday date for the given person.
### Add
Registers a new birthday.
- `name`: Name of the person
- `month`: Month of birth of the person
- `day`: Day of birth of the person
### Remove
Unregister a birthday date
- `name`: Name of the person
### Nuke
Remove every registered birthdays
