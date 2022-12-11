const charset = ["0","1","2","3","4","5","6","7","8","9","a","b","c","d","e","f"];

let array = [];
for (let i = 0; i < 100000; i++) {
    array.push(new RustData(generate_string(rand(16) + 1)));
}

array.sort((a, b) => a.cmp(b));

function generate_string(len) {
    let data = "";
    for (let i = 0; i < len; i++) {
        data = data + charset[rand(charset.length)]
    }
    return data;
}

function rand(max) {
    return Math.floor(Math.random() * max);
}
