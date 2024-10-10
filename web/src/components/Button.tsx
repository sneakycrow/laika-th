import { ReactNode } from "react"

type ButtonProps = {
    children: ReactNode
    onClick?: (event: React.MouseEvent<HTMLButtonElement>) => void
}

const Button = (props: ButtonProps) => {
    return (
        <button
            className="border-4 rounded-sm text-lg font-bold px-4 py-2 dark:border-white dark:bg-black dark:text-white bg-white text-black border-black"
            onClick={props.onClick}
        >
            {props.children}
        </button>
    )
}

export default Button
