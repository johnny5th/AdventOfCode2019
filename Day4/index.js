const inputL = 402328;
const inputH = 864247;

function password_valid(password, part) {
  if (password <= inputL || password >= inputH) {
    return false;
  }

  password = password.toString();

  let found_adjacent_pair = false;
  for (let i=0; i<password.length-1; i++) {
    if (password[i] > password[i+1]) {
      return false;
    }

    if (part == 1) {
      if (password[i] == password[i+1]) {
        found_adjacent_pair = true;
      }
    } else {
      if (password[i] != (typeof password[i-1] === 'undefined' ? 0 : password[i-1])
        && password[i] == password[i+1]
        && password[i] != (typeof password[i+2] === 'undefined' ? 0 : password[i+2])) {
        found_adjacent_pair = true;
      }
    }
  }

  if (!found_adjacent_pair) {
    return false;
  }

  return true;
}

let valid_p1_passwords = [];
let valid_p2_passwords = [];

for (let i=inputL; i<inputH; i++) {
  if (password_valid(i, 1)) {
    valid_p1_passwords.push(i);
  }

  if (password_valid(i, 2)) {
    valid_p2_passwords.push(i);
  }
}

console.log('P1 Passwords: ' + valid_p1_passwords.length);
console.log('P2 Passwords: ' + valid_p2_passwords.length);