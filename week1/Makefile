SUBDIRS = module1 vector-fruit-salad csv

.PHONY: all format lint test buld release $(SUBDIRS)

$(SUBDIRS):
	$(MAKE) -C $@ $(MAKECMDGOALS)

clean format lint test build release all: $(SUBDIRS)
