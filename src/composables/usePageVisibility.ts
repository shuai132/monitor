import {onMounted, onUnmounted, ref} from 'vue'

export function usePageVisibility() {
    const pageVisible = ref(true)

    const handleVisibilityChange = () => {
        pageVisible.value = !document.hidden
    }

    onMounted(() => {
        document.addEventListener('visibilitychange', handleVisibilityChange)
    })

    onUnmounted(() => {
        document.removeEventListener('visibilitychange', handleVisibilityChange)
    })

    return {pageVisible}
}
