import { useRouter } from 'tuono'

export default function Threads() {
    const router = useRouter()
    return <div>Thread list for board  {router.pathname}</div>
    
}