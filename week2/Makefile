SUBDIRS = fruit_vector_lab

.PHONY: all format lint test build release $(SUBDIRS)

$(SUBDIRS):
	$(MAKE) -C $@ $(MAKECMDGOALS)

clean format lint test build release all: $(SUBDIRS)
