<script>
export default {
  methods: {
    goToSettings() {
      this.$router.push('/settings')
    },
  },
}
</script>

<template>
  <h2>HomeView</h2>
  <button @click="goToSettings">Go to Settings</button>
</template>