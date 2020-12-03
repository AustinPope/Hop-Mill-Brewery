// Websocket js code from guide https://github.com/steadylearner/Chat
const emoji = require("node-emoji");
const hasEmoji = require("has-emoji");

const socket = new WebSocket("ws://127.0.0.1:7777/ws");

function getDateTime() {
  const now = new Date();
  const date =
    now.getFullYear() + "-" + (now.getMonth() + 1) + "-" + now.getDate();
  const time = now.getHours() + ":" + now.getMinutes() + ":" + now.getSeconds();
  const payload = date + " " + time;
  return payload;
}

function removeMessages() {
  const messages = document.getElementById("messages");
  while (messages.firstChild) {
    messages.removeChild(messages.firstChild);
  }
}

let open = false;

let userId = "";
let userInputs = [];

let server = [];

// Log when socket is open
socket.addEventListener("open", function (event) {
  console.log("Start to chat");
});

// clear the message
const clear = document.getElementById("clear");
clear.onclick = removeMessages;

// exit the chat
const exit = document.getElementById("exit");
exit.onclick = function () {
  socket.close();
};

// id - form, relevant to who typed the message
const form = document.getElementById("form");
form.onsubmit = function (event) {
  event.preventDefault();
  const input = document.getElementById("msg");

  if (input.value === "") {
    return;
  }

  if (input.value === "!clear") {
    removeMessages();
    input.value = "";
    return;
  }

  if (input.value === "!exit") {
    socket.close();
    return;
  }

  // can use database instead of localStorage
  const userInputWithTime = `${userId} typed ${
    input.value
  } at ${getDateTime()}`;
  userInputs.push(userInputWithTime);

  socket.send(`${userId}: ${input.value}`);
  input.value = "";

  setTimeout(
    () => window.scrollTo({ top: window.innerHeight, behavior: "auto" }),
    10
  );
};

socket.onmessage = function (event) {
  // can use database to save what server sent
  const messageFromServer = `Server ${event.origin} sent ${
    event.data
  } at ${getDateTime()}`;
  server.push(messageFromServer);

  if (userInputs[userInputs.length - 1] === "!warn") {
    alert("Warning sent to other users");
  }

  if (event.data.includes("!clearall")) {
    removeMessages();
    return;
  }

  if (event.data.includes("!exitall")) {
    socket.close();
    return;
  }

  if (event.data.includes("!x-opacity")) {
    const messages = document.getElementById("messages");
    if (messages.className === "x-opacity") {
      messages.className = "";
    } else {
      messages.className = "x-opacity";
    }
    return;
  }

  if (!open) {
    let separate = event.data.split(" ");
    userId = separate[0];

    const messages = document.getElementById("messages");
    const li = document.createElement("li");
    const p = document.createElement("p");

    let totalNumber = separate[separate.length - 1];
    if (totalNumber > 5) {
      p.textContent = "A maximum of 5 users is allowed.";
      p.className = "red-white";
      li.append(p);
      messages.append(li);
      socket.close();
      return;
    }

    open = true;

    p.textContent = `Your id is ${userId} and "Me" will be used in this page instead`;
    p.className = "blue";
    li.append(p);
    messages.append(li);
    return;
  } else {
    let fromServer = event.data;
    const beforePayload = fromServer.split(" ")[0];
    const authorOfMessage = beforePayload.slice(0, beforePayload.length - 1); // to get the id part of the message

    if (fromServer.includes(`!exclude ${userId}`)) {
      socket.close();
      return;
    }

    const messages = document.getElementById("messages");
    const li = document.createElement("li");

    // Give color and "Me" for a user when author of the messages is the user
    if (authorOfMessage === userId) {
      li.className = "red-white";
      fromServer = fromServer.replace(userId, "Me");
    }

    const includeEmoji = hasEmoji(emoji.emojify(fromServer));
    afterEmoji = includeEmoji ? emoji.emojify(fromServer) : fromServer;

    const p = document.createElement("p");
    p.append(afterEmoji);
    li.append(p);
    messages.append(li);
    return;
  }
};

socket.onclose = function (event) {
  const closeMessage =
    event.data === undefined
      ? "Server or another user closed the connection."
      : "WebSocket is closed.";
  const messages = document.getElementById("messages");

  const li = document.createElement("li");
  li.append(closeMessage);
  li.className = "blue";
  messages.append(li);

  // can use database API instead of localStorage
  localStorage.setItem("userInputs", `[${userInputs}]`);
  localStorage.setItem("server", `[${server}]`);
};
