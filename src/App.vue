<template>
  <h1>Hello Martini!</h1>
  <p>
    <strong>Current route path:</strong> {{ $route.fullPath }}
  </p>
  <nav>
    <RouterLink to="/">Go to Home</RouterLink>
    <RouterLink to="/settings">Go to Settings</RouterLink>
  </nav>
  <main>
    <RouterView />
  </main>
</template>

<style>
nav, main {
  border: 2px solid #000;
  margin-bottom: 10px;
  padding: 10px;
}

nav > a + a {
  margin-left: 10px;
}

h2 {
  border-bottom: 1px solid #ccc;
  margin: 0 0 20px;
}
</style>