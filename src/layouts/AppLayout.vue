<template>
  <header>
    <h1>Hello Martini!</h1>
    <p>
      <strong>Current route path:</strong> {{ $route.fullPath }}
    </p>
    <nav>
      <RouterLink to="/">Go to Home</RouterLink>
      <RouterLink to="/settings">Go to Settings</RouterLink>
    </nav>
  </header>

  <RouterView />
</template>
