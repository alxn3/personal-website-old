import Link from 'next/link';

const NavLinks: { name: string; link: string }[] = [
  {
    name: 'Home',
    link: '/',
  },
  {
    name: 'Fluid',
    link: '/fluid',
  },
];

const Navbar = () => {
  return (
    <nav className="flex items-center flex-wrap bg-gray-800 p-3 ">
      <Link href="/">
        <a className="inline-flex items-center p-2 mr-4">
          <span className="text-xl text-white font-bold uppercase tracking-wide">
            Alex Lin
          </span>
        </a>
      </Link>
      <div className="ml-auto inline-flex flex-row">
        {NavLinks.map((links) => (
          <Link href={links.link} key={links.link}>
            <a className="px-3 py-2 rounded text-gray-400 font-bold items-center justify-center hover:bg-gray-600 hover:text-white">
              {links.name}
            </a>
          </Link>
        ))}
      </div>
    </nav>
  );
};

export default Navbar;
