SDLLIB = $(shell sdl2-config --libs) $(LIBS)
SDLFLAGS = $(shell sdl2-config --cflags) -L/usr/X11R6/include

SRCS = hostcall.c screen.c input.c

OBJS = $(SRCS:.c=.o)

THIS=Makefile
LIBFLAGS += -lm

.c.o:
	$(CC) $(CFLAGS) $(INCFLAGS) $(SDLFLAGS) -c $<

default:
	make -f $(THIS) $(OBJS)
clean:
	rm -f *.o

# Use "make depends" to generate file dependencies:
depends: $(SRCS)
	gcc -MM $(CFLAGS) $(SRCS) > Makefile.dep

ifneq (,$(wildcard Makefile.dep))
include Makefile.dep
endif
