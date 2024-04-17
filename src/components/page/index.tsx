import { Title } from "@solidjs/meta";

type HomeProps = {
  title: string
}

export default function Home(props: HomeProps) {
  return (
    <main>
      <Title>{props.title}</Title>
      <h1>{props.title}</h1>
      <p>
        Home Page
      </p>
    </main>
  );
}
