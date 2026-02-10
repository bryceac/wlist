# wlist

**Author:** Bryce Campbell

**License:** See LICENSE

**Description:** CLI program written in Rust that can be used to create wishlist in HTML.

**Version:** 0.1.0

## Notes

This project is a <abbr tite="work in progress">WIP</abbr>. It currently only has a database that will act as the backend and some functions that are necessary to communicate with it propery.

I have no idea how long it will take me to get things working, but it should be simple enough at this stage.

### Version History

<dl>
<dt style="font-weight:bold">0.1.0</dt>
<dd>Initial release</dd>
</dl>

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

  <p>While I do not intend to have this program do all of those things, I do intend to have it generate the HTML with items linked up to their respective notes, so that I can focus on implementing the style rules necessary myself, such as making sure the link addresses are displayed on the prnted page and making sure items have enough space for navigation on a touch screen phone.</p>
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

Currently, I have been able to implement everything that I desire as of now.

However, if you think you can help make this program even better, feel free to fork this project and make a pull request.

### Support

While I have done a lot more programming in Rust these days, I still don't consider my abilities to be that great, so expect to be on your own.

However, I will try my best to help you as much as I can if you email me at the address below:

tonyhawk2100@gmail.com
