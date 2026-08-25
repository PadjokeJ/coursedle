const TYPE = [
    "course_id",
    "title",
    "language",
    "credits",
    "semester",
    "section"
];

let guesses = 0;

// Source - https://stackoverflow.com/a/47593316
// Posted by bryc, modified by community. See post 'Timeline' for change history
// Retrieved 2026-08-23, License - CC BY-SA 4.0

function sfc32(a, b, c, d) {
  return function() {
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

function createRow(course, correct) {
    guesses += 1;
    let tbody = document.getElementById("tbody");

    let row = document.createElement("tr");
    row.classList.add("guess");

    row.style.transition = "all 0.5s ease";

    let yippee = true;

    for (let i = 0; i < 6; i++) {
        let td = document.createElement("td");
        td.innerText = course[TYPE[i]];
        if (course[TYPE[i]] == correct[TYPE[i]]) {
            td.classList.add("correct");
        } else {
            td.classList.add("wrong");
            yippee = false;
        }
        row.appendChild(td);
    }

    if (yippee) {
        let v = document.getElementById("victory_screen");
        v.classList.remove("hidden");

        let a = document.getElementById("course_link");
        a.innerText = correct.title;
        a.setAttribute("href", `https://edu.epfl.ch${correct.link}`);
        
        let g = document.getElementById("num_guesses");
        g.innerText = guesses;
    }

    tbody.appendChild(row);
}

function getData() {
    return fetch("/static/data.json")
        .then((response) => response.json())
        .then((json) => {
            return json;
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

    if (lwc.length < 4) return;

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
});

sel.addEventListener("input", () => {
    inp.value = sel.value;
})

gue.addEventListener("click", async () => {
    let data = await DATA;

    let correct = await courseToday();

    createRow(data.courses[sel.value], data.courses[correct]);
});

async function courseToday() {
    let r = sfc32(((new Date().getDay() - 2) / 31) * 2 ** 32., (new Date().getMonth() / 31) * 2  ** 32., (new Date().getUTCFullYear() / 3000) * 2 ** 32., 2**32);

    for (let i = 0; i < 10; i++) {
        r();
    }

    let data = await DATA;

    let max = Object.keys(data.courses).length;

    let i = (r() * max) | 0;

    return Object.keys(data.courses)[i];
}
