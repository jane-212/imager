import request from '@/utils/http'

export default function get_images() {
    return request({
        url: '/image/',
        method: 'get'
    })
}