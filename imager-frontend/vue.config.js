const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  transpileDependencies: true,
  devServer: {
    proxy: {
      '/api': {
        target: 'http://124.222.172.51',
        changeOrigin: true,
        pathRewrite: {
          '^/api': '/api'
        }
      }
    },
  }
})