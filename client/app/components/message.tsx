export default function Messages(props: any) {
    return(
        <div className="flex flex-col h-20 w-[80%]">
            <div className="">
                {props.data.username}
            </div>
            <div>
                {props.data.content}
            </div>
        </div>
    )
}