let year = parse_int(variable::get("year"));
let day = parse_int(variable::get("day"));

let current_year = system::date().year;
if year < 2015 || year > current_year {
    abort("year must be 2015-" + current_year);
}

if day < 1 || day > 25 {
    abort("day must be 1-25");
}

let padded = "" + day;
if day < 10 {
    padded = "0" + padded;
}
variable::set("day_padded", padded);