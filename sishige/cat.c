// The origin of "cat" is conCATenate.

#include "regular_linux_programming.h"

static void	die(const char *s)
{
	perror(s);
	exit(1);
}

static void	do_cat(const char *path)
{
	int				fd;
	unsigned char	buf[BUFFER_SIZE];
	int				n;

	if (!path)
		fd = fileno(stdin);
	else
		fd = open(path, O_RDONLY);
	if (fd < 0)
		die(path);
	while (true)
	{
		n = read(fd, buf, sizeof(buf));
		if (n < 0)
			die(path);
		if (n == 0)
			break ;
		if (write(STDOUT_FILENO, buf, n) < 0)
			die(path);
	}
	if (close(fd) < 0)
		die(path);
}

int	main(int argc, char **argv)
{
	int	i;

	if (argc < 2)
	{
		do_cat(NULL);
		return (0);
	}
	i = 1;
	while (i < argc)
		do_cat(argv[i++]);
	return (0);
}

