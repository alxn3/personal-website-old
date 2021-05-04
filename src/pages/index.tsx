import Navbar from 'components/navbar';
import Head from 'next/head';

const Home = () => {
  return (
    <div>
      <Head>
        <title>Home | Alex Lin</title>
        <meta name="description" content="Alex Lin's personal website." />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <Navbar />
      </main>
    </div>
  );
};

export default Home;
