const TYPE = [
    "course_id",
    "title",
    "language",
    "credits",
    "semester",
    "section"
];

const FIRST_DAY = new Date(Date.parse("2026-08-25"));

const TODAY = function () {
    let a = new Date();
    a.setHours(FIRST_DAY.getHours()); // account for utc timezones hour shifts
    a.setMinutes(0);
    a.setSeconds(0);
    a.setMilliseconds(0);

    return a;
}();

let guesses = 0;

let progress = "";

let won = false;

let seen = [];

// Source - https://stackoverflow.com/a/47593316
// Posted by bryc, modified by community. See post 'Timeline' for change history
// Retrieved 2026-08-23, License - CC BY-SA 4.0

function sfc32(a, b, c, d) {
    return function () {
        a |= 0; b |= 0; c |= 0; d |= 0;
        let t = (a + b | 0) + d | 0;
        d = d + 1 | 0;
        a = b ^ b >>> 9;
        b = c + (c << 3) | 0;
        c = (c << 21 | c >>> 11);
        c = c + t | 0;
        return (t >>> 0) / 4294967296;
    }
}

// END paste

function lowOrHigh(n1, n2) {
    return (Number(n1) == Number(n2)) ? "" : (Number(n1) < Number(n2)) ? "⬆️" : "⬇️"
}

function compare(cat1, cat2) {
    console.log(`comparing ${cat1} and ${cat2}`)

    if (Number.isInteger(cat1)) {
        console.log("is number")

        return lowOrHigh(cat1, cat2);
    }

    if (cat1.startsWith("BA") || cat1.startsWith("MA")) {
        console.log("is start with")
        
        if (cat1.slice(0, 2) == cat2.slice(0, 2)) {
            let cats1 = cat1.slice(2);
            let cats2 = cat2.slice(2);

            return lowOrHigh(cats1, cats2);
        }

        if (cat1.startsWith("BA") && cat2.startsWith("MA")) {
            return "⬆️";
        } 

        return "⬇️";
    } 
    

    return "";
}

function createRow(course, correct) {
    if (seen.includes(course.course_id)) {
        return;
    }

    seen.push(course.course_id);

    guesses += 1;
    let tbody = document.getElementById("tbody");

    let row = document.createElement("tr");
    row.classList.add("guess");

    row.style.transition = "all 0.5s ease";

    let yippee = true;

    for (let i = 0; i < 6; i++) {
        let td = document.createElement("td");
        let n = course[TYPE[i]];
        console.log(!Number.isNaN(n));

        n += compare(n, correct[TYPE[i]]);

        td.innerText = n;
        if (n == correct[TYPE[i]]) {
            progress += "🟩"
            td.classList.add("correct");
        } else {
            progress += "🟥"
            td.classList.add("wrong");
            yippee = false;
        }
        row.appendChild(td);
    }

    progress += "<br>"

    if (yippee) {
        let v = document.getElementById("victory_screen");
        v.classList.remove("hidden");

        let a = document.getElementById("course_link");
        a.innerText = correct.title;
        a.setAttribute("href", `https://edu.epfl.ch${correct.link}`);

        let g = document.getElementById("num_guesses");
        g.innerText = guesses;

        document.getElementById("progress").innerHTML = progress;

        document.getElementById("day").innerText = Math.round((TODAY - FIRST_DAY) / (1000 * 60 * 60 * 24));

        inp.disabled = true;
        sel.disabled = true;

        won = true;
    }

    tbody.appendChild(row);
}

function getData() {
    const args = window.location.search;
    const params = new URLSearchParams(args);

    return fetch("/static/data.json")
        .then((response) => response.json())
        .then((json) => {
            if (!params.has("section")) {
                return json;
            }
            
        });
}

const DATA = getData();

const inp = document.getElementById("guess_input");
const sel = document.getElementById("course_id");
const gue = document.getElementById("submit_guess");

inp.addEventListener("keyup", async () => {
    sel.innerHTML = '';

    let courses = [];
    let data = await DATA;
    let dc = data.courses;

    let lwc = inp.value.toLowerCase();

    if (lwc.length < 3) return;

    for (const id in dc) {
        let course = dc[id];
        if (course.title.toLowerCase().startsWith(lwc) || id.toLowerCase().startsWith(lwc)) {
            courses.push(course);
        }
    }

    for (const i in courses) {
        const course = courses[i];
        let opt = document.createElement("option");
        opt.value = course.course_id;
        opt.innerText = `${course.title} - ${course.course_id}`;
        sel.appendChild(opt);
    }

    if (event.key == "Enter") {
        await check_correct();
    }
});

sel.addEventListener("input", () => {
    inp.value = sel.value;
})

async function check_correct() {
    if (won) {
        return;
    }

    let data = await DATA;

    let correct = await courseToday();

    createRow(data.courses[sel.value], data.courses[correct]);

    sel.innerHTML = '';
    inp.value = '';
}

gue.addEventListener("click", async () => {
    await check_correct();
});

async function courseToday() {
    let r = sfc32(((TODAY.getDay() - 2) / 31) * 2 ** 32., (TODAY.getMonth() / 31) * 2 ** 32., (TODAY.getUTCFullYear() / 3000) * 2 ** 32., 2 ** 32);

    for (let i = 0; i < 10; i++) {
        r();
    }

    let data = await DATA;

    let max = Object.keys(data.courses).length;

    let i = (r() * max) | 0;

    return Object.keys(data.courses)[i];
}

document.getElementById("share").addEventListener("click", () => {
    let t = document.getElementById("progress").innerText;

    t = "coursedle.EPFL.cc day #" + document.getElementById("day").innerText + "\n" + t;

    navigator.clipboard.writeText(t);

    // Alert the copied text
    alert("Copied to clipboard!");
})
