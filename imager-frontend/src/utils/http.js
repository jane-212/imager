import axios from 'axios'

let service = axios.create({
    baseURL: '/api',
    timeout: 5000,
})

service.interceptors.response.use(
    response => {
        const res = response.data

        if (res.code != 0) {
            return Promise.reject(new Error('connection error'))
        } else {
            return res
        }
    }
)

export default service