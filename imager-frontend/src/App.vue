<template>
  <div>
    <MainHeader></MainHeader>
    <MainContainer></MainContainer>
  </div>
</template>

<script>
import MainHeader from './components/MainHeader.vue'
import MainContainer from './components/MainContainer.vue'

export default {
  name: 'App',
  components: {
    MainHeader,
    MainContainer
  }
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>
