<template>
  <div class="header">
    <img class="logo" alt="logo" src="../assets/logo.png">
  </div>
</template>

<script>
export default {
  data: function() {
    return {}
  }
}
</script>

<style scoped>
.header {
  position: fixed;
  top: 0;
  width: 100vw;
  height: 4vh;
  text-align: center;
  background-color: black;
}

.logo {
  height: 3vh;
}
</style>
