---
name: rules-code-frontend
description: "The rules for the code of a page in a browser: the navigation, the requests, the actions, the rendering and the stability of the layout. Use when you write or change a page, a component, a link or a form, when a page sends a request or polls, when you write an action that a user starts, when you write the state of a component, when a page renders on the server and in the browser, when you write a loading state or a skeleton, when a value appears or disappears on a page, and when a page is slow or jumps."
---

# Code Frontend

Obey each rule of this file each time that you write the code of a page and each time that you change the code of a page.

## 1. Prefetch each link to a read-only page

Prefetch the page of each link on hover. Prefetch the code of the page with its data, thus the next page needs no second request for its code.

Prefetch a clickable surface that is not a link on the press of the pointer, not on hover, because a scroll moves the pointer over each row of a list.

Keep each layout that two pages or more share mounted across a visit between those pages, such as the persistent layout of the router. Do not render the layout inside each page, because a visit then mounts the layout again, loses its state and repeats its requests.

## 2. Keep each page that a GET request opens read-only

Write no data in the handler of a GET request. When a GET page must write, such as a page that marks a notification as read, name that page in a rule file of the project, and turn off the prefetch of each link to it. Add each new page of that kind to the same rule file.

## 3. Flush each stale page after a write

After an action writes data, remove each cached or prefetched page that shows that data, thus the next visit shows the new state and not the old state.

## 4. Send one request for one intent

Send one request for one action of the user. Send no second request that only gives back a URL or a value that the first response can hold.

Send the data of the first paint with the page. Do not fetch that data in an effect after the component mounts.

Send the data that is slow and that the first paint does not need, such as a chart below the fold or a list in a secondary panel, as a deferred value of the router: the page shows at once, and the router asks for that value in one request after the page. Show a skeleton of section 9 in its place.

## 5. Ask only for the data that changes

Give each reload and each poll the names of the values that it needs, and ask for no other value. Keep the old data on the screen until the new data arrives.

Load shared data that changes slowly, such as the current user or a list of options, one time, and keep it across visits. Give it a time limit when it must refresh.

## 6. Update the screen before the server answers

Show the result of each action at once, and roll it back when the server gives a failure. Use the tool of the project for each kind of state, such as the optimistic update of the router, the form helper of the framework, or a client store for a counter or a chat. Name each of these tools in a rule file of the project when the project uses two of them or more.

Wait for the server in two cases only: the result holds a value that only the server has, such as a new identifier, and a validation error must show next to its field.

Mark each update that renders a slow part of a page as a transition, such as a list that a filter changes or a chart that a period picker changes, thus the input answers at once and the old content stays on the screen until the new content is ready.

## 7. Derive each value during the render

Calculate a value from the props and the state during the render. Do not copy a prop or a state into a second state with an effect.

Declare no component inside a different component. Give no index as the key of an item in a list that can change its order, add an item or remove an item.

When the framework has a compiler that memoizes each component, such as the React Compiler, turn it on in the build of the project. When the project compiles each component with automatic memoization, write no manual memoization. When it does not, memoize only a calculation that a measurement shows to be slow, and a value that a memoized child receives.

## 8. Render the same output on the server and in the browser

Read each value that differs between the server and the browser, such as the time, the time zone, the size of the window and the local storage, through a subscription that gives the server its own snapshot. Do not read it during the first render.

Keep no data of one user in the state of a module on the server. Reset each store of a module for each request.

## 9. Keep the layout still

Give each skeleton the exact size of the content that replaces it. Show a skeleton only after about 300 ms, thus a fast response shows no flash.

Give a button the same width in each state, such as the label "Save" and the label "Saving". Show each number with tabular figures. Reserve the space of each part of a page that appears on a condition, such as an error message or a badge.

## 10. Check the page in a browser

The check needs two items: a browser tool of the client, such as the Claude in Chrome extension, and access to the application on the local computer. When one of the two items is not available, stop the task and ask the user to connect the extension or to start the application. Do not replace the check with a request with `curl` or with a test, and do not report the task as finished without the check.

Open each page that your change touches in a browser, in each viewport and each theme that the project names. Measure the shift of the layout for each new action, and give each shift a target of 0.

Read the requests of the network tab: one prefetch for each hover, no prefetch for a page of section 2, and one request for each action.

## 11. Report

Name each link that your change adds, and say whether it prefetches.

Name each GET page that writes data, and give the rule file that names it.

Name each action that your change adds, give the update that it shows at once, and give its rollback.

Name each value that your change defers, and give the skeleton that takes its place.

Name each page that you opened in a browser, with the viewports, the themes and the shift of the layout that you measured.

Write this report in the last message of your answer. Write no sentence for a rule of this file that your work did not touch.
