# wlist

**Author:** Bryce Campbell

**License:** See LICENSE

**Description:** CLI program written in Rust that can be used to create wishlist in HTML.

**Version:** 0.1.0

## Notes

This project is a <abbr tite="work in progress">WIP</abbr>. It currently only has a database that will act as the backend and some functions that are necessary to communicate with it propery.

I have no idea how long it will take me to get things working, but it should be simple enough at this stage.

### Questions

1. <dl><dt style="font-weight:bold">There are already so many different options for wishlists. Why create this program?</dt>
  <dd>
  <p>While I have seen many different options out there, since the goal of a wishlist is to share it with others, None of them really do what I'd like to do, 
  which is generate a list that is friendly for those that want to print off the list.</p>

  <p>Currently, I have do things as fllows:</p>

  <ul>
  <li>create list</li>
  <li>generate HTML</li>
  <li>clean up HTML</li>
  <li>add style rules to HTML to make it print and screen friendly</li>
  <li>add notes if necessary and link them to items.</li>
  </ul>

  <p>While I do not intend to have this program do all of those things, I do intend to have it generate the HTML with items linked up to their respective notes, 
    so that I can focus on implementing the style rules necessary myself, such as making sure the link addresses are displayed on the prnted page 
    and making sure items have enough space for navigation on a touch screen phone.</p>
  </dd>
  </dl>

2. <dl><dt style="font-weight:bold">Can I import my already existing list into this program?</dt>
  <dd>
  <p>At present, you cannot, but the model being used for this application has support for importing data from two formats, which are as follows:</p>

  <ul>
  <li>JSON</li>
  <li>TSV</li>
  </ul>

  <p>Of the two formats, only JSON has support for notes to be included with items, due to the complicated nature of how notes are stored in the model.</p>

  <p>As such, TSV is only suitable for importing new items or updating everything about items in the wishlist except for notes.</p>
  </dd>
</dl>

3. <dl><dt style="font-weight:bold">Will I be able to edit my notes?</dt>
  <dd>
  <p>This is something I am currently thinkig about implementing in some capacity, as I have written functions to do that.</p>
  <p>However, unlike items, I think that if I add the ability to import them, the ability will be limited to JSON, so that things are not as complicated.</p>
  </dd>
</dl>

### Contributing

Considering that this project is in its beginning phase at the moment, there is plenty to do at the moment, such as:

* implement subcommand to import items to wishlist
* implement method to export wishlist
* implement subcommand update items in the wishlist
* implement subcommand generate the HTML with list ordered from highest priority to low priority

I am sure that I will eventually be able to tick these off at some point, but if you want to help out, feel free to fork this project and issue a pull request.
