all: toc clean-tmp fmt


# generate table of content.
# See:
# https://github.com/ekalinin/github-markdown-toc
toc:
	gh-md-toc --insert replication-algo.md


# gh-md-toc generates tmp files.
clean-tmp:
	rm *.md.orig.*
	rm *.md.toc.*

fmt:
	# nmp install -g prettier
	prettier --write --print-width 80 --prose-wrap preserve **/*.md
