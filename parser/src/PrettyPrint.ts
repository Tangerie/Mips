export default function PrettyPrint(...data : any[]) {
    for(const d of data) {
        console.log(JSON.stringify(d, null, 4));
    }
}